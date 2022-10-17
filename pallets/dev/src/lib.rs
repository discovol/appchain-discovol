#![cfg_attr(not(feature = "std"), no_std)]

pub  use pallet::*;

#[frame_support::pallet]
pub  mod pallet {

	use bs58;
	use frame_support::sp_runtime::AccountId32;
	use sp_std::vec::Vec;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::SaturatedConversion,
		traits::{Currency, ExistenceRequirement, ReservableCurrency},
	};

	use frame_system::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Token was initialized by user
		Initialized(T::AccountId),
		/// Tokens successfully transferred between users
		Transfer(T::AccountId, u64), // (from, to, value)
	}

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_sender)]
	pub(super) type Sender<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		NotSender,
		ExistSender,
		InvalidSender,
		PayError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//
		#[pallet::weight(10_000)]
		pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			//
			let sender = ensure_signed(_origin)?;

			ensure!(!Self::is_init(), <Error<T>>::ExistSender);

			<Sender<T>>::insert(&sender, 1);

			Init::<T>::put(true);

			Self::deposit_event(Event::Initialized(sender));

			Ok(().into())
			//
		}
		//
		/// Transfer tokens from one account to another
		#[pallet::weight(10_000)]
		pub fn transfer(
			_origin: OriginFor<T>,
			to: T::AccountId,
			value: u64,
		) -> DispatchResultWithPostInfo {
			//
			ensure!(Self::is_init(), <Error<T>>::NotSender);

			let sender = ensure_signed(_origin)?;

			let exist = Self::get_sender(&sender);

			log::info!("\n\nexist: {:?}\n\n", exist);

			ensure!(exist > 0, <Error<T>>::InvalidSender);

			<Balances<T>>::insert(&to, value);

			Self::deposit_event(Event::Transfer(to, value));

			Ok(().into())
			//
		}

		//
		/// Transfer tokens from one account to another
		#[pallet::weight(10_000)]
		pub fn transfer2(
			_origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] pay_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			//
			log::info!("\n\npay_amount: {:?}\n\n", pay_amount);

			let value = pay_amount.saturated_into::<u64>();

			log::info!("\n\nvalue: {:?}\n\n", value);

			let value2 = (value / 10) * 2;

			let pay_amount2 = value2.saturated_into();

			log::info!("\n\nbalance: {:?}\n\n", pay_amount2);

			let sender = ensure_signed(_origin)?;

			if T::Currency::transfer(&sender, &to, pay_amount2, ExistenceRequirement::KeepAlive)
				.is_err()
			{
				return Err(Error::<T>::InvalidSender)?;
			}

			ensure!(Self::is_init(), <Error<T>>::NotSender);

			let exist = Self::get_sender(&sender);

			log::info!("\n\nexist: {:?}\n\n", exist);

			ensure!(exist > 0, <Error<T>>::InvalidSender);

			<Balances<T>>::insert(&to, value);

			Self::deposit_event(Event::Transfer(to, value));

			Ok(().into())
			//
		}

		//
		/// Transfer tokens from one account to another
		#[pallet::weight(100_000)]
		pub fn transfer3(
			_origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] pay_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			//
			let sender = ensure_signed(_origin)?;

			log::info!("\n\npay_amount: {:?}\n\n", pay_amount);

			let pay_value = pay_amount.saturated_into::<u128>();

			log::info!("\n\npay_value: {:?}\n\n", pay_value);

			let pay_value1 = (pay_value / 10) * 2;

			let pay_amount1 = pay_value1.saturated_into();

			log::info!("\n\npay_amount1: {:?}\n\n", pay_amount1);

			if T::Currency::transfer(&sender, &to, pay_amount1, ExistenceRequirement::KeepAlive)
				.is_err()
			{
				return Err(Error::<T>::PayError)?;
			}

			let pay_value2 = (pay_value / 10) * 4;

			let pay_amount2 = pay_value2.saturated_into();

			log::info!("\n\npay_amount2: {:?}\n\n", pay_amount2);

			if T::Currency::transfer(&sender, &to, pay_amount2, ExistenceRequirement::KeepAlive)
				.is_err()
			{
				return Err(Error::<T>::PayError)?;
			}

			let pay_value3 = (pay_value / 10) * 4;

			let pay_amount3 = pay_value3.saturated_into();

			log::info!("\n\npay_amount3: {:?}\n\n", pay_amount3);

			if T::Currency::transfer(&sender, &to, pay_amount3, ExistenceRequirement::KeepAlive)
				.is_err()
			{
				return Err(Error::<T>::PayError)?;
			}

			Ok(().into())
			//
		}

		#[pallet::weight(10_000)]
		pub fn batch(origin: OriginFor<T>, dests: Vec<T::AccountId>, tokens: u128) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(origin)?;

			let dests_len = dests.len();

			ensure!(dests_len < 101, <Error<T>>::TooManyDests);

			let token: u128 = 100_000_000_000_000;

			let amount: BalanceOf<T> = (token * tokens).saturated_into();

			let dest_enum = dests.clone().into_iter().enumerate();

			for (index, dest) in dest_enum {
				//

				log::info!("\n\ndest{:?}: {:?} {:?}\n\n", &index, &dest, &amount);

				let r = T::SpreadCurrency::transfer(&sender, &dest, &amount, ExistenceRequirement::KeepAlive);

				log::info!("\n\nResult {:?}\n\n", Some(r));

				// Self::deposit_event(Event::ItemCompleted);
			}

			Self::deposit_event(Event::BatchCompleted(sender, dests, tokens));

			// Ok(Some(base_weight + weight).into())

			Ok(().into())

			//
		}

		//
	}
}
