#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use sp_std::vec::Vec;

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

		//BatchCompleted(Vec<T::AccountId>, Vec<T::AccountId>),
		BatchCompleted(T::AccountId, u64),
		//
	}

	#[pallet::storage]
	#[pallet::getter(fn is_init)]
	pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_sender)]
	pub(super) type Sender<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery>;

	#[pallet::storage]
	pub type Invites<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, T::BlockNumber, OptionQuery>;

	#[pallet::storage]
	// pub type Inviteds<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (T::AccountId, T::BlockNumber), ValueQuery>;
	pub type Inviteds<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (T::AccountId, T::BlockNumber)>;

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

		TooFewInvites,

		TooManyInvites,

		InvalidArgument,
		//
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn init(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(origin)?;

			ensure!(!Self::is_init(), <Error<T>>::ExistSender);

			<Sender<T>>::insert(&sender, 1);

			Init::<T>::put(true);

			Self::deposit_event(Event::Initialized(sender));

			Ok(().into())

			//
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,10))]
		pub fn batch_insert(origin: OriginFor<T>, invites: Vec<T::AccountId>, inviteds: Vec<T::AccountId>) -> DispatchResultWithPostInfo {
			//

			ensure!(Self::is_init(), <Error<T>>::NotSender);

			let sender = ensure_signed(origin)?;

			let exist = Self::get_sender(&sender);

			ensure!(exist > 0, <Error<T>>::InvalidSender);

			let invites_len = invites.len();

			let inviteds_len = inviteds.len();

			ensure!(invites_len == inviteds_len, <Error<T>>::InvalidArgument);

			ensure!(invites_len > 0, <Error<T>>::TooFewInvites);

			// ensure!(invites_len < 1001, <Error<T>>::TooManyInvites);

			let current_block = <frame_system::Pallet<T>>::block_number();

			for i in 0..invites_len {
				//

				<Invites<T>>::insert(&invites[i], &inviteds[i], &current_block);

				<Inviteds<T>>::insert(&inviteds[i], (&invites[i], &current_block));

				//
			}

			//Self::deposit_event(Event::BatchCompleted(invites, inviteds));
			Self::deposit_event(Event::BatchCompleted(sender, invites_len as u64));

			Ok(().into())

			//
		}

		//
	}
}
