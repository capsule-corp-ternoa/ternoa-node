use super::mock::*;
use crate::tests::mock;
use frame_system::RawOrigin;
use ternoa_common::traits::CapsulesTrait;

#[test]
fn is_capsulized() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let alice_nft = help::create_capsule_fast(alice.clone());
            let unknown_nft = 1001;

            assert_eq!(TernoaCapsules::is_capsulized(alice_nft), true);
            assert_eq!(TernoaCapsules::is_capsulized(unknown_nft), false);
        })
}
