#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};

	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		//
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		//
	}

	#[pallet::event]
	//#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		//
		Initialized(T::AccountId),

		Transfer(T::AccountId, u64),
		//
	}

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_sender)]
	pub(super) type Sender<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		//
		NotSender,

		ExistSender,

		InvalidSender,

		PayError,
		//
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
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

		//#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		#[pallet::weight(1_000)]
		pub fn transfer(_origin: OriginFor<T>, to: T::AccountId, value: u64) -> DispatchResultWithPostInfo {
			//

			ensure!(Self::is_init(), <Error<T>>::NotSender);

			let sender = ensure_signed(_origin)?;

			let exist = Self::get_sender(&sender);

			ensure!(exist > 0, <Error<T>>::InvalidSender);

			<Balances<T>>::insert(&to, value);

			Self::deposit_event(Event::Transfer(to, value));

			Ok(().into())

			//
		}

		//
	}
}
