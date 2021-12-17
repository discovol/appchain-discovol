#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	use frame_support::traits::{Currency, ExistenceRequirement, ReservableCurrency};

	use frame_support::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[rustfmt::skip]
	#[pallet::config]
	pub trait Config: frame_system::Config {

		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

	}

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[rustfmt::skip]
	pub enum Event<T: Config> {

		SpreadUrlCreated(T::AccountId, Vec<u8>, Vec<u8>, BalanceOf<T>, u16),
	
	}

	#[pallet::error]
	#[rustfmt::skip]
	pub enum Error<T> {
		
		UrlSpreaded,
		
		SpreadPayFail,

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type SpreadUrls<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Blake2_128Concat,
		T::AccountId,
		(T::BlockNumber, Vec<u8>, BalanceOf<T>, T::AccountId, T::AccountId, u16),
		OptionQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	#[rustfmt::skip]
	impl<T: Config> Pallet<T> {

		// 传播URL
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_spread(origin: OriginFor<T>, hash: Vec<u8>, url: Vec<u8>, #[pallet::compact] pay_amount: BalanceOf<T>, pay_dest: T::AccountId, relation: T::AccountId, score: u16) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			let current_block = <frame_system::Pallet<T>>::block_number();

			if SpreadUrls::<T>::contains_key(&hash, &sender) {
		
				return Err(Error::<T>::UrlSpreaded)?;
		
			}

			if T::Currency::transfer(&sender, &pay_dest, pay_amount.clone(), ExistenceRequirement::KeepAlive).is_err() {
			
				return Err(Error::<T>::SpreadPayFail)?;
			
			}

			SpreadUrls::<T>::insert(&hash, &sender, (&current_block, &url, pay_amount.clone(), &pay_dest, &relation, &score));

			Self::deposit_event(Event::SpreadUrlCreated(sender, hash, url, pay_amount, score));

			Ok(().into())
		
		}
	
	}
}
