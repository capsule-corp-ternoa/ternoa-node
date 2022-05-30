// Copyright 2022 Capsule Corp (France) SAS.
// This file is part of Ternoa.

// Ternoa is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Ternoa is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Ternoa.  If not, see <http://www.gnu.org/licenses/>.

//! Runtime versioning values. Used when pushing runtime upgrades, in most cases
//! you only need to increment `spec_version`.

use crate::RUNTIME_API_VERSIONS;
use sp_runtime::create_runtime_str;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	/// Identifies the different Substrate runtimes. There'll be at least polkadot and node.
	/// A different on-chain spec_name to that of the native runtime would normally result
	/// in node not attempting to sync or author blocks.
	spec_name: create_runtime_str!("mainnet"),

	/// Name of the implementation of the spec. This is of little consequence for the node
	/// and serves only to differentiate code of different implementation teams. For this
	/// codebase, it will be parity-polkadot. If there were a non-Rust implementation of the
	/// Polkadot runtime (e.g. C++), then it would identify itself with an accordingly different
	/// `impl_name`.
	impl_name: create_runtime_str!("mainnet"),

	/// `authoring_version` is the version of the authorship interface. An authoring node
	/// will not attempt to author blocks unless this is equal to its native runtime.
	authoring_version: 1,

	/// Version of the runtime specification. A full-node will not attempt to use its native
	/// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	/// `spec_version` and `authoring_version` are the same between Wasm and native.
	spec_version: 2,

	/// Version of the implementation of the specification. Nodes are free to ignore this; it
	/// serves only as an indication that the code is different; as long as the other two versions
	/// are the same then while the actual code may be different, it is nonetheless required to
	/// do the same thing.
	/// Non-consensus-breaking optimizations are about the only changes that could be made which
	/// would result in only the `impl_version` changing.
	impl_version: 0,

	// Is a list of supported runtime APIs along with their versions.
	apis: RUNTIME_API_VERSIONS,

	/// All existing dispatches are fully compatible when this number doesn't change. If this
	/// number changes, then `spec_version` must change, also.
	///
	/// This number must change when an existing dispatchable (module ID, dispatch ID) is changed,
	/// either through an alteration in its user-level semantics, a parameter
	/// added/removed/changed, a dispatchable being removed, a module being removed, or a
	/// dispatchable/module changing its index.
	///
	/// It need *not* change when a new module is added or when a dispatchable is added.
	transaction_version: 1,

	/// Version of the state implementation used by this runtime.
	/// Use of an incorrect version is consensus breaking.
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
/// Typically used when building the `Executor` in our node.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}
