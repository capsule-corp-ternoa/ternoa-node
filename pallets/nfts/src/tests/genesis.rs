use super::mock::*;
use crate::GenesisConfig;
use crate::NFTDetails;
use frame_support::traits::GenesisBuild;

#[test]
fn register_nfts() {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    GenesisConfig::<Test> {
        nfts: vec![(ALICE, NFTDetails::new(vec![1], 0, false))],
        series: vec![],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        assert_eq!(NFTs::nft_id_generator(), 1);
        assert_eq!(NFTs::data(0).owner, ALICE);
        assert_eq!(NFTs::data(0).details, NFTDetails::new(vec![1], 0, false));
        assert_eq!(NFTs::data(0).locked, false);
        assert_eq!(NFTs::data(0).sealed, false);
    });
}
