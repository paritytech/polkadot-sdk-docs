use frame::{
	prelude::*,
	runtime::{prelude::*, runtime_apis},
};
use frame_pallets::currency::pallet as currency_pallet;

// TODO: this is not optimal
#[frame::macros::use_attr]
use frame::deps::frame_support::derive_impl;

#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("node-template"),
	impl_name: create_runtime_str!("node-template"),
	authoring_version: 1,
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
);

construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = Extrinsic,
	{
		System: frame_system,
		Currency: currency_pallet,
	}
);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();

	type Version = Version;
}

impl currency_pallet::Config for Runtime {}

use frame::runtime::runtime_types_common::{self, ExtrinsicOf, HeaderOf};
use frame_support::parameter_types;

type Block = runtime_types_common::BlockOf<Runtime, SignedExtra>;
type Header = runtime_types_common::HeaderOf<Block>;
type Extrinsic = runtime_types_common::ExtrinsicOf<Block>;

pub type RuntimeExecutive =
	Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

impl_runtime_apis! {
	impl runtime_apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			RuntimeExecutive::execute_block(block)
		}

		fn initialize_block(header: &Header) {
			RuntimeExecutive::initialize_block(header)
		}
	}
	impl runtime_apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl runtime_apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: ExtrinsicOf<Block>) -> ApplyExtrinsicResult {
			RuntimeExecutive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> HeaderOf<Block> {
			RuntimeExecutive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicOf<Block>> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl runtime_apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicOf<Block>,
			block_hash: <Runtime as frame_system::Config>::Hash,
		) -> TransactionValidity {
			RuntimeExecutive::validate_transaction(source, tx, block_hash)
		}
	}

	impl runtime_apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderOf<Block>) {
			RuntimeExecutive::offchain_worker(header)
		}
	}

	impl runtime_apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, runtime_apis::KeyTypeId)>> {
			Default::default()
		}
	}

	impl runtime_apis::AccountNonceApi<
		Block,
		<Runtime as frame_system::Config>::AccountId,
		<Runtime as frame_system::Config>::Index,
	> for Runtime {
		fn account_nonce(account: <Runtime as frame_system::Config>::AccountId) -> <Runtime as frame_system::Config>::Index {
			System::account_nonce(account)
		}
	}
}
