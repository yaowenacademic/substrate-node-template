#![cfg_attr(not(feature = "std"), no_std)]


/// A module for proof of existence


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;


    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber), 
        ValueQuery
    >;

//    #[pallet::metadata(T::AccountId = "AccountId")]

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>, 
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {

            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Proofs::<T>::insert(&claim,(&sender, current_block));

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            
            Ok(().into())

        }

        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>, 
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let sender  = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            
            //let (owner: <T as Config>::AccountId, _) = Proofs::<T>::get(key: &claim).ok_or(err: Error::<T>::ClaimNotExist)?;
            let (owner, _) = Proofs::<T>::get(&claim);
            

            ensure!(owner == sender, Error::<T>::NotClaimOwner);


            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo {

            let sender  = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            
            let (owner, _) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Proofs::<T>::insert(&claim, (dest, current_block));

            Ok(().into())

        }
    }


}

