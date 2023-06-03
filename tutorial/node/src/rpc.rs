//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use jsonrpsee::RpcModule;
use runtime::{AccountId, Index, OpaqueBlock};
use sc_transaction_pool_api::TransactionPool;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;

pub use sc_rpc_api::DenyUnsafe;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: sp_api::ProvideRuntimeApi<
		// TODO: bloody hell why does runtime::OpaqueBlock does not work here? maybe some issue
		// with the dependecies eg two sp-runtimes being around..
		frame::runtime::runtime_types_generic::Block<
			frame::runtime::runtime_types_generic::Header<u32, frame::primitives::BlakeTwo256>,
			frame::runtime::runtime_types_generic::OpaqueExtrinsic,
		>,
	>,
	C: HeaderBackend<OpaqueBlock> + HeaderMetadata<OpaqueBlock, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: BlockBuilder<OpaqueBlock>,
	P: TransactionPool + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<OpaqueBlock, AccountId, Index>,
{
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	// use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	// module.merge(TransactionPayment::new(client).into_rpc())?;

	Ok(module)
}
