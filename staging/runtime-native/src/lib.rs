use frame::arithmetic::Perbill;
use frame::deps::frame_system::limits::{BlockLength, BlockWeights};
use frame::prelude::*;
use frame::runtime::runtime_types_common::HeaderOf;
use frame::runtime::{self as frame_runtime, create_runtime_str};
use frame::runtime::{NativeVersion, RuntimeVersion};
use frame_runtime::{
	impl_runtime_apis, runtime_apis::*, runtime_types_common::ExtrinsicOf, ApplyExtrinsicResult,
	CheckInherentsResult, InherentData, OpaqueMetadata,
};

#[frame::runtime::runtime_version]
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

const MILLISECS_PER_BLOCK: u64 = 6000;
const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
const HOURS: BlockNumber = MINUTES * 60;
const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

type BlockNumber = u32;
type AccountId = frame_runtime::runtime_types_common::AccountId;
type Balance = u128;
type Hash = frame::primitives::H256;

type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
);

use frame_runtime::runtime_types_common;
type Header = runtime_types_common::Header;
type Block = runtime_types_common::BlockOf<RuntimeCall, SignedExtra>;
type Extrinsic = runtime_types_common::ExtrinsicOf<Block>;
// type SignedPayload = runtime_types_generic::SignedPayload<RuntimeCall, ()>;

pub type Executive = frame::runtime::Executive<
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
		NodeBlock = Block,
		UncheckedExtrinsic = Extrinsic,
	{
		System: frame_system,
	}
);

parameter_types! {
pub const Version: RuntimeVersion = VERSION;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub RuntimeBlockWeights: BlockWeights =
		BlockWeights::with_sensible_defaults(
			Weight::from_parts(2u64 * frame::runtime::weights::constants::WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
			Perbill::from_percent(75),
		);
	pub RuntimeBlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, Perbill::from_percent(75));
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = frame::traits::AccountIdLookup<AccountId, ()>;
	type Index = u32;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = frame::primitives::BlakeTwo256;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = frame::traits::ConstU32<2000>;
	type DbWeight = frame_runtime::db_weights::RocksDbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = ();
	type SystemWeightInfo = ();
	type SS58Prefix = frame::traits::ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

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
		fn apply_extrinsic(extrinsic: ExtrinsicOf<Block>) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> HeaderOf<Block> {
			Executive::finalize_block()
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

	impl frame_runtime::runtime_apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicOf<Block>,
			block_hash: Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl frame_runtime::runtime_apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderOf<Block>) {
			Executive::offchain_worker(header)
		}
	}
}
