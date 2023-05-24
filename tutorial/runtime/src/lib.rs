#![cfg_attr(not(feature = "std"), no_std)]

use frame::{
	prelude::*,
	runtime::{
		self as frame_runtime, create_runtime_str, impl_runtime_apis, runtime_apis::*,
		runtime_types_common, ApplyExtrinsicResult, CheckInherentsResult, InherentData,
		OpaqueMetadata, RuntimeVersion,
	},
};

#[frame::runtime::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("frame-tutorial-runtime"),
	impl_name: create_runtime_str!("frame-tutorial-runtime"),
	authoring_version: 1,
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type OnSetCode = ();
	type BaseCallFilter = frame::traits::Everything;

	type Header = Header;

	type AccountId = runtime_types_common::AccountId;
	type BlockNumber = u32;
	type Index = u32;
	type Version = Version;

	type BlockWeights = ();
	type BlockLength = ();
	type Hash = frame::primitives::H256;
	type Hashing = frame::primitives::BlakeTwo256;
	type Lookup = frame::traits::AccountIdLookup<Self::AccountId, ()>;
	type BlockHashCount = frame::traits::ConstU32<250>;
	type MaxConsumers = frame::traits::ConstU32<16>;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type DbWeight = ();
}

// Our custom pallet. Always import it such that the final path refers to the `pallet` module.
use frame_pallets::currency::pallet as currency;
impl currency::Config for Runtime {}

type SignedExtensions = runtime_types_common::SystemSignedExtensionsOf<Runtime>;
type OpaqueBlock = runtime_types_common::OpaqueBlockOf<Runtime>;
type Block = runtime_types_common::BlockOf<Runtime, SignedExtensions>;
type Header = runtime_types_common::HeaderOf<Block>;
type Extrinsic = runtime_types_common::ExtrinsicOf<Block>;

type Executive = frame::runtime::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

frame::runtime::construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = OpaqueBlock,
		UncheckedExtrinsic = Extrinsic,
	{
		System: frame_system,
		Currency: currency,
	}
);

type AccountId = <Runtime as frame_system::Config>::AccountId;
type Index = <Runtime as frame_system::Config>::Index;

impl_runtime_apis! {
	impl frame_runtime::runtime_apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &Header) {
			Executive::initialize_block(header)
		}
	}
	impl frame_runtime::runtime_apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> frame::std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl frame_runtime::runtime_apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl frame_runtime::runtime_apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: Extrinsic,
			block_hash: <Runtime as frame_system::Config>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl frame_runtime::runtime_apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &Header) {
			Executive::offchain_worker(header)
		}
	}

	impl frame_runtime::runtime_apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, frame_runtime::runtime_apis::KeyTypeId)>> {
			Default::default()
		}
	}

	impl frame_runtime::runtime_apis::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}
}
