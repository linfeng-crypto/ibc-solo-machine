use num_rational::Ratio;
use rust_decimal::Decimal;
use solo_machine_core::ibc::core::ics24_host::identifier::{Identifier, PortId};
use std::str::FromStr;
use std::time::Duration;
use tendermint::block::Height as BlockHeight;

#[derive(Default, Debug, Clone)]
pub struct AddChainPage {
    /// the basic config when
    pub basic: AddChainBasicPage,
    /// the advanced page
    pub advanced: AddChainAdvancedPage,
}

#[derive(Debug, Clone)]
pub struct AddChainBasicPage {
    /// grpc address of IBC enabled chain
    pub input_grpc_address: String,
    /// RPC address of IBC enabled chain
    pub input_rpc_address: String,
}

impl Default for AddChainBasicPage {
    fn default() -> Self {
        Self {
            input_rpc_address: "http://0.0.0.0:9090".into(),
            input_grpc_address: "http://0.0.0.0:26657".into(),
        }
    }
}

//TODO: move this to app_config and save to file
#[derive(Clone, Debug)]
pub struct AddChainAdvancedPage {
    /// Fee amount, default 1000
    pub fee_amount: Decimal,
    /// Fee denom
    pub fee_denom: Identifier,
    /// Gas Limit, default 300000
    pub gas_limit: u64,
    /// Trust level
    pub trust_level: Ratio<u64>,
    /// Trusting period
    pub trusting_period_days: u64,
    /// RPC timeout: Duration, default 3 secs
    pub max_clock_drift: Duration,
    /// rpc_timeout: Duration
    pub rpc_timeout: Duration,
    /// Diversifier used in transactions for chain
    pub diversifier: String,
    /// Port ID used to create connection with chain
    pub port_id: PortId,
    /// Trusted height of the chain, can get from the rpc
    pub trusted_height: Option<BlockHeight>,
    /// Block hash at trusted height of the chain, can get from the rpc
    pub trusted_hash: Option<[u8; 32]>,
}

impl Default for AddChainAdvancedPage {
    fn default() -> Self {
        Self {
            fee_amount: "1000".parse().unwrap(),
            fee_denom: Identifier::from_str("basecro").unwrap(),
            gas_limit: 300000,
            trust_level: Ratio::from_str("1/3").unwrap(),
            trusting_period_days: 14,
            max_clock_drift: Duration::from_secs(3),
            rpc_timeout: Duration::from_secs(60),
            diversifier: "solo-machine-diversifier".into(),
            port_id: PortId::from_str("transfer").unwrap(),
            trusted_height: None,
            trusted_hash: None,
        }
    }
}
