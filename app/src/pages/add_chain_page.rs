use crate::state::SavedState;
use crate::style::style_input_title;

use anyhow::Result;
use iced::{button, text_input, Align, Button, Element, Row, Text, TextInput};
use num_rational::Ratio;
use rust_decimal::Decimal;
use solo_machine::signer::SignerRegistrar;
use solo_machine_core::ibc::core::ics24_host::identifier::{Identifier, PortId};
use solo_machine_core::model::{ChainConfig, Fee};
use solo_machine_core::service::ChainService;
use solo_machine_core::utils::parse_trusted_hash;
use solo_machine_core::ToPublicKey;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use tendermint::block::Height as BlockHeight;
use tokio::runtime::Runtime;

/// chain message
#[derive(Debug, Clone)]
pub enum NewChainMessage {
    InputGrpcAddress(String),
    InputRpcAddress(String),
    InputTrustedHeight(String),
    InputTrustedHash(String),
    CreateNewChain,
}

#[derive(Default, Debug, Clone)]
pub struct AddChainPage {
    /// the basic config when
    pub basic: AddChainBasicPage,
    /// the advanced page
    pub advanced: AddChainAdvancedPage,
}

#[derive(Debug, Clone)]
pub struct AddChainBasicPage {
    pub input_grpc_address: text_input::State,
    pub input_rpc_address: text_input::State,
    pub input_trusted_height: text_input::State,
    pub input_trusted_hash: text_input::State,
    pub ok_button: button::State,
    /// grpc address of IBC enabled chain
    pub grpc_address: String,
    /// RPC address of IBC enabled chain
    pub rpc_address: String,
    /// Trusted height of the chain, can get from the rpc
    pub trusted_height: String,
    /// Block hash at trusted height of the chain, can get from the rpc
    pub trusted_hash: String,
}

impl AddChainBasicPage {
    pub fn update(&mut self, message: NewChainMessage) -> Result<()> {
        match message {
            NewChainMessage::InputRpcAddress(s) => {
                self.rpc_address = s;
            }
            NewChainMessage::InputGrpcAddress(s) => {
                self.grpc_address = s;
            }
            NewChainMessage::InputTrustedHash(s) => {
                self.trusted_hash = s;
            }
            NewChainMessage::InputTrustedHeight(s) => {
                self.trusted_height = s;
            }
            NewChainMessage::CreateNewChain => {
                log::info!("pressed create new chain button");
                let trusted_hash = parse_trusted_hash(&self.trusted_hash).unwrap_or_default();
                let trusted_height = BlockHeight::from_str(&self.trusted_height)?;
                let page = AddChainAdvancedPage::default();

                let config = ChainConfig {
                    grpc_addr: self.grpc_address.clone(),
                    rpc_addr: self.rpc_address.clone(),
                    fee: Fee {
                        amount: page.fee_amount,
                        denom: page.fee_denom,
                        gas_limit: page.gas_limit,
                    },
                    trust_level: page.trust_level,
                    trusting_period: Duration::from_secs(page.trusting_period_days * 24 * 60 * 60),
                    max_clock_drift: page.max_clock_drift,
                    rpc_timeout: page.rpc_timeout,
                    diversifier: page.diversifier,
                    port_id: page.port_id,
                    trusted_height,
                    trusted_hash,
                };

                log::info!("create new chain");
                let run_time = Runtime::new().unwrap();
                let _ = run_time
                    .block_on(async move {
                        if !page.signer_path.exists() {
                            log::error!("");
                            anyhow::bail!("signer file is not exists");
                        }
                        let signer = SignerRegistrar::try_from(page.signer_path)
                            .unwrap()
                            .unwrap()?;
                        log::info!("get signer success");
                        let db_pool = SavedState::db_pool().await?;
                        let chain_service = ChainService::new(db_pool);
                        log::info!("add chain config: {:?}", config);
                        chain_service
                            .add(&config, &signer.to_public_key().unwrap().encode())
                            .await
                            .map(|_| ())
                    })
                    .map_err(|e| log::error!("create new chain error: {:?}", e));
            }
        }
        Ok(())
    }

    pub fn view(&mut self) -> Vec<Element<NewChainMessage>> {
        let mut result = vec![];
        let rpc_address = self.rpc_address.clone();
        let grpc_address = self.grpc_address.clone();
        let trusted_height = self.trusted_height.clone();
        let trusted_hash = self.trusted_hash.clone();
        // rpc input
        let rpc_text_input = TextInput::new(
            &mut self.input_rpc_address,
            &rpc_address,
            &mut self.rpc_address,
            NewChainMessage::InputRpcAddress,
        )
        .padding(15)
        .size(20);
        let row = Row::new()
            .spacing(40)
            .align_items(Align::Center)
            .push(style_input_title("rpc address"))
            .push(rpc_text_input)
            .into();
        result.push(row);

        // grpc input
        let grpc_text_input = TextInput::new(
            &mut self.input_grpc_address,
            &grpc_address,
            &mut self.grpc_address,
            NewChainMessage::InputGrpcAddress,
        )
        .padding(15)
        .size(20);
        let row = Row::new()
            .spacing(40)
            .align_items(Align::Center)
            .push(style_input_title("grpc address"))
            .push(grpc_text_input)
            .into();
        result.push(row);

        // trusted_height
        let height_text_input = TextInput::new(
            &mut self.input_trusted_height,
            &trusted_height,
            &mut self.trusted_height,
            NewChainMessage::InputTrustedHeight,
        )
        .padding(15)
        .size(20);
        let row = Row::new()
            .spacing(40)
            .align_items(Align::Center)
            .push(style_input_title("trusted height"))
            .push(height_text_input)
            .into();
        result.push(row);

        // trusted_hash
        let hash_text_input = TextInput::new(
            &mut self.input_trusted_hash,
            &trusted_hash,
            &mut self.trusted_hash,
            NewChainMessage::InputTrustedHash,
        )
        .padding(15)
        .size(20);
        let row = Row::new()
            .spacing(40)
            .align_items(Align::Center)
            .push(style_input_title("trusted hash"))
            .push(hash_text_input)
            .into();
        result.push(row);

        // ok button
        let label = Text::new("OK").size(16);
        let ok_button = Button::new(&mut self.ok_button, label)
            .on_press(NewChainMessage::CreateNewChain)
            .padding(8);
        result.push(ok_button.into());

        result
    }
}

impl Default for AddChainBasicPage {
    fn default() -> Self {
        Self {
            input_rpc_address: text_input::State::focused(),
            input_grpc_address: text_input::State::new(),
            input_trusted_height: text_input::State::new(),
            input_trusted_hash: text_input::State::new(),
            ok_button: button::State::new(),
            rpc_address: "http://0.0.0.0:26657".into(),
            grpc_address: "http://0.0.0.0:9090".into(),
            trusted_height: "".into(),
            trusted_hash: "".into(),
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
    pub signer_path: PathBuf,
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
            signer_path: PathBuf::from_str("/tmp/libmnemonic_signer.dylib").unwrap(),
        }
    }
}
