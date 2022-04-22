#![cfg_attr(not(feature="std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*
	};

	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config>{

		///Event emitted when a proof has been claim.[who claim]
		ClaimCreated(T::AccountId, Vec<u8>),

		///Event emitted when a claim is revoked by the owmer.[who, claim]
		ClaimRevoked(T::AccountId, Vec<u8>),

		///Event emitted when a claim is transfer[from, to, claim] 
		ClaimTransfer(T::AccountId, T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {

		/// The proof has already been claim.
		ProofAlreadyClaim,

		/// The proof does not exist, so it cannot be reovked.
		NotSuchProof,

		/// The proof is claim by another account, so caller can't revoke it.
		NotProofOfOwner,
	}

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub(super) type Proofs<T: Config> = StorageMap<
	  _,
	  Blake2_128Concat,
	  Vec<u8>,
	  (T::AccountId, T::BlockNumber)
	>;

	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// Create a claim 
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,
		    claim: Vec<u8>
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyClaim);

			let current_block = <frame_system::Pallet<T>>::block_number();

			Proofs::<T>::insert(&claim, (sender.clone(), current_block));

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

		///Remove a claim
		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NotSuchProof)?;

			ensure!(sender == owner, Error::<T>::NotProofOfOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		///Transfer a claim
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			to_account:T::AccountId
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NotSuchProof)?;

			ensure!(sender == owner, Error::<T>::NotProofOfOwner);

			Proofs::<T>::remove(&claim);

			let current_block = <frame_system::Pallet<T>>::block_number();

			Proofs::<T>::insert(&claim, (to_account.clone(), current_block));

			Self::deposit_event(Event::ClaimTransfer(sender, to_account, claim));

			Ok(().into())
		}
	}
}