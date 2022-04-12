pub use ternoa_runtime_common::constants::currency;

pub mod time {
	use ternoa_core_primitives::BlockNumber;
	use ternoa_runtime_common::{constants::time::*, prod_or_fast};

	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = prod_or_fast!(4 * HOURS, 4 * MINUTES);
}
