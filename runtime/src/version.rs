//! Runtime versioning values. Used when pushing runtime upgrades, in most cases
//! you only need to increment `spec_version`.

use crate::RUNTIME_API_VERSIONS;
use sp_runtime::create_runtime_str;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("ternoa"),
    impl_name: create_runtime_str!("capsule-corp-node"),

    // The version of the authorship interface. An authoring node will not attempt to
    // author blocks unless this is equal to its native runtime.
    authoring_version: 1,

    // Per convention: if the runtime behavior changes, increment spec_version
    // and set impl_version to 0. If only runtime
    // implementation changes and behavior does not, then leave spec_version as
    // is and increment impl_version.
    spec_version: 40,

    // The version of the implementation of the specification. Nodes are
    // free to ignore this; it serves only as an indication that the code is
    // different; as long as the other two versions are the same then while the actual
    // code may be different, it is nonetheless required to do the same thing.
    // Non-consensus-breaking optimizations are about the only changes that could be
    // made which would result in only the impl_version changing.
    impl_version: 0,

    // The version of the extrinsics interface. This number must be updated in the
    // following circumstances: extrinsic parameters (number, order, or types) have
    // been changed; extrinsics or pallets have been removed; or the pallet order in
    // the construct_runtime! macro or extrinsic order in a pallet has been changed.
    // If this number is updated, then the spec_version must also be updated.
    transaction_version: 4,

    // Is a list of supported runtime APIs along with their versions.
    apis: RUNTIME_API_VERSIONS,
};

/// The version information used to identify this runtime when compiled natively.
/// Typically used when building the `Executor` in our node.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
