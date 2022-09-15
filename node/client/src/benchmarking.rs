use sc_cli::Result;
use sp_api::Encode;
use sp_core::{Pair, H256};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{OpaqueExtrinsic, SaturatedConversion};

use std::{sync::Arc, time::Duration};

use crate::*;

/// Generates extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder {
	client: Arc<Client>,
}

impl RemarkBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<Client>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		with_client! {
			self.client.as_ref(), client, {
				use runtime::{Call, SystemCall};

				let call = Call::System(SystemCall::remark { remark: vec![] });
				let signer = Sr25519Keyring::Bob.pair();

				let period = ternoa_runtime_common::BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
				let genesis = client.usage_info().chain.best_hash;
				let best_block = client.chain_info().best_number;

				Ok(client.sign_call(call, nonce, period, genesis, signer, best_block))
			}
		}
	}
}

/// Helper trait to implement [`frame_benchmarking_cli::ExtrinsicBuilder`].
///
/// Should only be used for benchmarking since it makes strong assumptions
/// about the chain state that these calls will be valid for.
trait BenchmarkCallSigner<Call: Encode + Clone, Signer: Pair> {
	/// Signs a call together with the signed extensions of the specific runtime.
	///
	/// Only works if the current block is the genesis block since the
	/// `CheckMortality` check is mocked by using the genesis block.
	fn sign_call(
		&self,
		call: Call,
		nonce: u32,
		period: u64,
		genesis: H256,
		acc: Signer,
		best_block: u32,
	) -> OpaqueExtrinsic;
}

#[cfg(feature = "mainnet")]
impl BenchmarkCallSigner<mainnet_runtime::Call, sp_core::sr25519::Pair>
	for FullClient<mainnet_runtime::RuntimeApi, MainnetExecutorDispatch>
{
	fn sign_call(
		&self,
		call: mainnet_runtime::Call,
		nonce: u32,
		period: u64,
		genesis: H256,
		acc: sp_core::sr25519::Pair,
		best_block: u32,
	) -> OpaqueExtrinsic {
		use mainnet_runtime as runtime;

		let extra: runtime::SignedExtra = (
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(sp_runtime::generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(0),
		);

		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis.clone(),
				genesis,
				(),
				(),
				(),
			),
		);

		let signature = payload.using_encoded(|p| acc.sign(p));
		runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(acc.public()).into(),
			ternoa_core_primitives::Signature::Sr25519(signature.clone()),
			extra,
		)
		.into()
	}
}

#[cfg(feature = "alphanet")]
impl BenchmarkCallSigner<alphanet_runtime::Call, sp_core::sr25519::Pair>
	for FullClient<alphanet_runtime::RuntimeApi, AlphanetExecutorDispatch>
{
	fn sign_call(
		&self,
		call: alphanet_runtime::Call,
		nonce: u32,
		period: u64,
		genesis: H256,
		acc: sp_core::sr25519::Pair,
		best_block: u32,
	) -> OpaqueExtrinsic {
		use alphanet_runtime as runtime;

		let extra: runtime::SignedExtra = (
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(sp_runtime::generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(0),
		);

		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis.clone(),
				genesis,
				(),
				(),
				(),
			),
		);

		let signature = payload.using_encoded(|p| acc.sign(p));
		runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(acc.public()).into(),
			ternoa_core_primitives::Signature::Sr25519(signature.clone()),
			extra,
		)
		.into()
	}
}

/// Generates inherent data for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub fn inherent_benchmark_data() -> Result<InherentData> {
	let mut inherent_data = InherentData::new();
	let d = Duration::from_millis(0);
	let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

	timestamp
		.provide_inherent_data(&mut inherent_data)
		.map_err(|e| format!("creating inherent data: {:?}", e))?;
	Ok(inherent_data)
}
