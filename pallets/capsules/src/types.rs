use ternoa_primitives::nfts::NFTId;

use sp_std::vec::Vec;

pub type CapsuleLedger<Balance> = Vec<(NFTId, Balance)>;
