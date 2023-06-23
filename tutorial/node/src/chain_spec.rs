use runtime::{RuntimeGenesisConfig, SystemConfig, WASM_BINARY};
use sc_service::{ChainType, Properties};

pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

fn props() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".to_string(), 0.into());
	properties.insert("tokenSymbol".to_string(), "UNIT".into());
	properties
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	Ok(ChainSpec::from_genesis(
		"Polkadot-SDK-Tutorial-Development",
		"pst-dev",
		ChainType::Development,
		move || testnet_genesis(wasm_binary),
		vec![],
		None,
		None,
		None,
		Some(props()),
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(wasm_binary: &[u8]) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig { code: wasm_binary.to_vec() },
		..Default::default()
	}
}
