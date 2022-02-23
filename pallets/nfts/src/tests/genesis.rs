use super::mock::*;
use crate::{GenesisConfig, NFTData};
use frame_support::traits::GenesisBuild;

#[test]
fn register_nfts() {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let nft_id = 100;
	let mint_fee = 10;
	let data = NFTData::new_default(ALICE, vec![1], vec![48]);

	GenesisConfig::<Test> {
		nfts: vec![(nft_id, data.clone())],
		series: vec![],
		nft_mint_fee: mint_fee,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		assert_eq!(NFTs::nft_id_generator(), nft_id + 1);
		assert_eq!(NFTs::series_id_generator(), 0);
		assert_eq!(NFTs::data(nft_id), Some(data));
		assert_eq!(NFTs::nft_mint_fee(), mint_fee);
	});
}
