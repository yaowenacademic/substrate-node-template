use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn create_claim_works() {

    new_test_ext().execute_with(|| {

        let claim: Vec<u8> = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (1,frame_system::Pallet::<Test>::block_number())
        );
    })
}