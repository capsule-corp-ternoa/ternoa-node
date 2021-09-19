use super::mock::*;
use crate::{MarketplaceId, MarketplaceInformation, MarketplaceType};
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};
use frame_support::Blake2_128Concat;
use frame_system::Config as FSConfig;

/*
frame_support::generate_storage_alias!(
    Marketplace,  MarketplaceCount => Value<MarketplaceId>
);

frame_support::generate_storage_alias!(
    Marketplace,  MarketplaceOwners<T: FSConfig> => Map<(Blake2_128Concat, MarketplaceId), T::AccountId>
);

#[test]
fn upgrade_from_v4_to_v5() {
    ExtBuilder::default().build().execute_with(|| {
        StorageVersion::put::<Marketplace>(&StorageVersion::new(4));

        MarketplaceCount::put(3);
        MarketplaceOwners::<Test>::insert(1, ALICE);
        MarketplaceOwners::<Test>::insert(2, BOB);
        let market_id_gen = 2;

        let weight = <Marketplace as OnRuntimeUpgrade>::on_runtime_upgrade();

        let market_1 =
            MarketplaceInformation::<Test>::new(MarketplaceType::Public, 0, ALICE, Vec::default());
        let market_2 =
            MarketplaceInformation::<Test>::new(MarketplaceType::Public, 0, BOB, Vec::default());

        assert!(MarketplaceCount::get().is_none());
        assert_eq!(MarketplaceOwners::<Test>::iter().count(), 0);
        assert_eq!(MarketplaceIdGenerator::<Test>::get(), market_id_gen);
        assert_eq!(
            Marketplaces::<Test>::iter().count() as u32,
            (market_id_gen + 1)
        );
        assert_eq!(Marketplaces::<Test>::get(1), Some(market_1));
        assert_eq!(Marketplaces::<Test>::get(2), Some(market_2));
        assert_eq!(StorageVersion::get::<Marketplace>(), 5);

        assert_ne!(weight, 0);
    })
} */

mod v6 {
    use super::*;
    use crate::migrations::v6::v5::MarketplaceInformation as v5MarketplaceInformation;
    use crate::Marketplaces as v6Marketplaces;

    frame_support::generate_storage_alias!(
        Marketplace,  Marketplaces<T: FSConfig> => Map<(Blake2_128Concat, MarketplaceId), v5MarketplaceInformation<T>>
    );

    #[test]
    fn upgrade_from_v5_to_v6() {
        ExtBuilder::default().build_v6_migration().execute_with(|| {
            StorageVersion::put::<Marketplace>(&StorageVersion::new(5));

            Marketplaces::<Test>::insert(
                0,
                v5MarketplaceInformation {
                    kind: MarketplaceType::Public,
                    commission_fee: Default::default(),
                    owner: ALICE,
                    allow_list: Default::default(),
                },
            );

            Marketplaces::<Test>::insert(
                1,
                v5MarketplaceInformation {
                    kind: MarketplaceType::Public,
                    commission_fee: Default::default(),
                    owner: BOB,
                    allow_list: Default::default(),
                },
            );

            let weight = <Marketplace as OnRuntimeUpgrade>::on_runtime_upgrade();

            let market_1 = MarketplaceInformation::<Test>::new(
                MarketplaceType::Public,
                0,
                ALICE,
                Vec::default(),
                "Ternoa Marketplace".into(),
            );
            let market_2 = MarketplaceInformation::<Test>::new(
                MarketplaceType::Public,
                0,
                BOB,
                Vec::default(),
                "User Marketplace".into(),
            );

            assert_eq!(v6Marketplaces::<Test>::get(0), Some(market_1));
            assert_eq!(v6Marketplaces::<Test>::get(1), Some(market_2));
            assert_eq!(StorageVersion::get::<Marketplace>(), 6);

            assert_ne!(weight, 0);
        })
    }
}

#[test]
fn upgrade_from_latest_to_latest() {
    ExtBuilder::default().build().execute_with(|| {
        let weight = <Marketplace as OnRuntimeUpgrade>::on_runtime_upgrade();
        assert_eq!(weight, 0);
    })
}
