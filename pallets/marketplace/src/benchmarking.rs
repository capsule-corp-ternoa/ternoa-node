use crate::{Call, Module, Trait};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn list() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_list::<Test>());
        });
    }

    #[test]
    fn unlist() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_unlist::<Test>());
        });
    }

    #[test]
    fn buy() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_unlist::<Test>());
        });
    }
}
