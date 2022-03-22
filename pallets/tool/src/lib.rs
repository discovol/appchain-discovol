#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use sp_std::vec::Vec;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::SaturatedConversion,
		traits::{Currency, ExistenceRequirement, ReservableCurrency},
	};

	use frame_system::pallet_prelude::*;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::event]
	//#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		//
		BatchCompleted(T::AccountId, Vec<T::AccountId>, Vec<u32>),
		//
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		//
		TooFewDests,

		TooManyDests,

		InvalidArgument,
		//
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,10))]
		pub fn batch_transfer(origin: OriginFor<T>, dests: Vec<T::AccountId>, tokens: Vec<u32>) -> DispatchResultWithPostInfo {
			//

			let sender = ensure_signed(origin)?;

			let dests_len = dests.len();

			let tokens_len = tokens.len();

			ensure!(dests_len == tokens_len, <Error<T>>::InvalidArgument);

			ensure!(dests_len > 0, <Error<T>>::TooFewDests);

			ensure!(dests_len < 101, <Error<T>>::TooManyDests);

			let token: u128 = 100_000_000_000_000;

			for i in 0..dests_len {
				//

				let amount: BalanceOf<T> = (token * tokens[i] as u128).saturated_into();

				let _ = T::Currency::transfer(&sender, &dests[i], amount, ExistenceRequirement::KeepAlive);

				//
			}

			Self::deposit_event(Event::BatchCompleted(sender, dests, tokens));

			Ok(().into())

			//
		}

		//
	}
}
