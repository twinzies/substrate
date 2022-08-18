#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
use frame_support::{
	codec::{Decode, Encode, MaxEncodedLen},
	traits::{
		Currency,
		ReservableCurrency,
	},
	weights::Weight,
	scale_info::TypeInfo,
	sp_runtime::{
		RuntimeDebug,
	},
};

use pallet_identity::IdentityField;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub use pallet::*;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

/// Interface required for identity verification.
pub trait IdentityVerifier<AccountId> {
	/// Function that returns whether an account has an identity registered with the identity provider.
	fn has_identity(who: &AccountId, fields: u64) -> bool;
}

#[derive(Copy, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Vote{
	Aye,
	Nay,
	Abstain,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The origin which may forcibly set or remove a proposal.
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		// The VoteIndex assigned to a proposal upon creation.
		type VoteIndex: Get<u64>;

		type VoteQty: Get<u64>;

		/// The minimum time a proposal must be alive before being called.
		#[pallet::constant]
		type MinTime: Get<u32>;

		/// Proposal fee.
		#[pallet::constant]
		type ProposalFee: Get<BalanceOf<Self>>;

		/// Running count of a proposal
		type Count: Get<u64>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// write logic here
			Ok(())
		}
	}
}
