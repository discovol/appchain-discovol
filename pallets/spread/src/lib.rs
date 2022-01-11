#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::sp_runtime::traits::Zero;

	use frame_support::sp_runtime::SaturatedConversion;

	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	use frame_support::traits::{Currency, ExistenceRequirement, ReservableCurrency};

	use frame_support::pallet_prelude::*;

	type BalanceOf<T> = <<T as Config>::SpreadCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_register::Config {
		//

		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type SpreadCurrency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		//
	}

	#[pallet::event]
	//#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		//
		Initialized(T::AccountId),

		SpreadUrlCreated(T::AccountId, Vec<u8>, Vec<u8>, BalanceOf<T>, u8),
		//
	}

	#[pallet::error]
	pub enum Error<T> {
		//
		ExistFund,

		NotFund,

		UrlSpreaded,

		UrlNotRegistered,

		PayFundFail,

		PayRegisterFail,

		PayRelationFail,

		InvalidHash,
		//
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_fund)]
	pub(super) type Fund<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	pub type SpreadUrls<T: Config> = StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, T::AccountId, (T::BlockNumber, Vec<u8>, BalanceOf<T>, T::AccountId, u8), OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn init(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(_origin)?;

			ensure!(!Self::is_init(), <Error<T>>::ExistFund);

			Fund::<T>::put(&sender);

			Init::<T>::put(true);

			Self::deposit_event(Event::Initialized(sender));

			Ok(().into())

			//
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,4))]
		pub fn create_spread(origin: OriginFor<T>, hash: Vec<u8>, url: Vec<u8>, relation: T::AccountId, score: u8) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(origin)?;

			ensure!(hash.len() == 32, <Error<T>>::InvalidHash);

			ensure!(Self::is_init(), <Error<T>>::NotFund);

			let fund = Self::get_fund();

			ensure!(!SpreadUrls::<T>::contains_key(&hash, &sender), <Error<T>>::UrlSpreaded);

			let (register, url_block, _, _, _) = pallet_register::Pallet::<T>::get_url(&hash);

			ensure!(!url_block.is_zero(), <Error<T>>::UrlNotRegistered);

			let current_block = <frame_system::Pallet<T>>::block_number();

			let dime: u128 = 10_000_000_000_000;

			let amount = dime * 10 + dime * (score as u128 - 5);

			let x: BalanceOf<T> = amount.saturated_into();

			let x2: BalanceOf<T> = (amount / 10 * 2).saturated_into();

			let x4: BalanceOf<T> = (amount / 10 * 4).saturated_into();

			let x6: BalanceOf<T> = (amount / 10 * 6).saturated_into();

			if register != sender {
				ensure!(T::SpreadCurrency::transfer(&sender, &register, x4.clone(), ExistenceRequirement::KeepAlive).is_ok(), <Error<T>>::PayRegisterFail);
			}

			if SpreadUrls::<T>::contains_key(&hash, &relation) {
				ensure!(T::SpreadCurrency::transfer(&sender, &fund, x2, ExistenceRequirement::KeepAlive).is_ok(), <Error<T>>::PayFundFail);

				ensure!(T::SpreadCurrency::transfer(&sender, &relation, x4, ExistenceRequirement::KeepAlive).is_ok(), <Error<T>>::PayRelationFail);
			} else {
				ensure!(T::SpreadCurrency::transfer(&sender, &fund, x6, ExistenceRequirement::KeepAlive).is_ok(), <Error<T>>::PayFundFail);
			}

			SpreadUrls::<T>::insert(&hash, &sender, (&current_block, &url, x.clone(), &relation, &score));

			Self::deposit_event(Event::SpreadUrlCreated(sender, hash, url, x, score));

			Ok(().into())

			//
		}

		//
	}
}
