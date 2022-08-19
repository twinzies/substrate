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

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub use pallet::*;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[derive(Copy, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum VoteTypes{
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
		type VoteIndex: Get<u64> + Clone;

		type VoteQty: Get<u64>;

		/// The minimum time a proposal must be alive before being called.
		#[pallet::constant]
		type MinTime: Get<u32>;

		/// Proposal fee.
		#[pallet::constant]
		type ProposalFee: Get<BalanceOf<Self>>;

		/// Running count of a proposal
		type Count: Get<u64>;

		// Weight information for extrinsics in this pallet.
		// todo!: define weights type WeightInfo: WeightInfo;
	}

	/// Struct for a Vote
	// pub struct Vote{}
	// 	voter: u64, // Should be AccountId but it is impossible to do simple things in here.
	// 	vote: VoteTypes,
	// 	qty: u64,
	// }
	/// The lookup table for all proposals
	#[pallet::storage]
	pub(super) type AllProposals<T: Config> =
		StorageMap<_, Blake2_256, T::VoteIndex, T::Count>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A Proposal was created.
		ProposalCreated,
		/// A vote was cast. 
		VoteCast,
		/// A proposal was called and funds released.
		ProposalCalled,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		GeneralError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn create_proposal(origin: OriginFor<T>, proposal: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Take ProposalFee.
			// Insert proposal into storage
			// Hash the proposal and return as VoteIndex
			Self::deposit_event(Event::<T>::ProposalCreated);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn cast_vote(origin: OriginFor<T>, proposal: u64, vote: VoteTypes, qty: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Check if AccountId has adequate balance.
			// insert into storage the qty and type (Aye / Nay) of votes.
			Self::deposit_event(Event::<T>::VoteCast);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn call_proposal(origin: OriginFor<T>, proposal: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// ensure the sender AccountId is the creator of the proposal.
			Self::deposit_event(Event::<T>::ProposalCalled);
			Ok(())
		}
	}
}
