#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_system::pallet_prelude::*;

	use frame_support::sp_runtime::SaturatedConversion;
	use sp_std::vec::Vec;

	use frame_support::traits::{Currency, ExistenceRequirement, ReservableCurrency};

	use frame_support::pallet_prelude::*;

	use sp_io::hashing::blake2_256;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		//
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		//
	}

	#[pallet::event]
	//#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		//
		Initialized(T::AccountId),

		RegisterUrlCreated(T::AccountId, Vec<u8>, Vec<u8>, BalanceOf<T>),
		//
	}

	#[pallet::error]
	pub enum Error<T> {
		//
		ExistFund,

		NotFund,

		UrlRegistered,

		RegisterPayFail,

		InvalidHash,

		InvalidUrl,
		//
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_fund)]
	pub(super) type Fund<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn get_url)]
	pub type RegisterUrls<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber, Vec<u8>, BalanceOf<T>, T::AccountId)>;

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

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,2))]
		pub fn create_register(origin: OriginFor<T>, hash: Vec<u8>, url: Vec<u8>) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(origin)?;

			ensure!(url.len() > 9 && url.len() < 501, <Error<T>>::InvalidUrl);

			let url_hash: Vec<u8> = blake2_256(&url).into();

			ensure!(hash == url_hash, <Error<T>>::InvalidHash);

			ensure!(Self::is_init(), <Error<T>>::NotFund);

			//let fund = Self::get_fund();
			let fund = Self::get_fund().expect("Fund must initialized");

			ensure!(!RegisterUrls::<T>::contains_key(&hash), <Error::<T>>::UrlRegistered);

			let pay_value: u128 = 1_000_000_000_000_000;

			let pay_amount: BalanceOf<T> = pay_value.saturated_into();

			ensure!(T::Currency::transfer(&sender, &fund, pay_amount.clone(), ExistenceRequirement::KeepAlive,).is_ok(), <Error::<T>>::RegisterPayFail);

			let current_block = <frame_system::Pallet<T>>::block_number();

			RegisterUrls::<T>::insert(&hash, (&sender, &current_block, &url, &pay_amount, &fund));

			Self::deposit_event(Event::RegisterUrlCreated(sender, hash, url, pay_amount));

			Ok(().into())

			//
		}
	}
}
