use iced::{Container, Element, HorizontalAlignment, Length, Text};
use solo_machine_core::ibc::core::ics24_host::identifier::ChainId;

use crate::error::AppError;
use crate::pages::add_chain_page::NewChainMessage;
use crate::pages::main_page::Filter;
use crate::state::SavedState;

/// chain message
#[derive(Debug, Clone)]
pub enum ChainMessage {
    /// when press action button
    DoAction(ChainId),
    /// when press disconnect button
    DoClose(ChainId),
    /// when press details button, show the chain details
    ShowDetailInfo(ChainId),
    /// Set to Active Status
    SetActive,
    /// Set to Closed
    SetClosed,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// the main page, show all the chains db
    ChainMessage(usize, ChainMessage),
    NewChainMessage(NewChainMessage),
    /// Filter changed
    FilterChanged(Filter),
    /// input when add chain
    InputAddChainConfig(AddChainConfig),
    /// advanced input when add chain
    InputAddChainConfigAdcanced(AddChainConfigAdvanced),
    /// when press add chain button
    PressAddChain,
    /// when press new connection button
    PressNewConnection(String),
    /// when press mint chain button
    PressMintChain,
    /// when press burn token button
    PressBurnToken,
    /// saved config or chains to file and db
    Saved,
    /// loaded config and chains from file and db
    Loaded(std::result::Result<SavedState, AppError>),
    /// exit app
    Exit,
}

/// config when add chain
#[derive(Debug, Clone)]
pub enum AddChainConfig {
    /// grpc address of IBC enabled chain, default http://0.0.0.0:9090
    InputGrpcAddress(String),
    /// RPC address of IBC enabled chain, default http://0.0.0.0:26657
    InputRpcAddress(String),
}

/// advanced settings when add chain
#[derive(Clone, Debug)]
pub enum AddChainConfigAdvanced {
    /// Fee amount, default 1000
    InputFeeAmount(String),
    /// Fee denom
    InputFeeDenom(String),
    /// Gas Limit, default 300000
    InputGasLimit(u64),
    /// Trust level
    InputTrustLevel(u64),
    /// Trusting period, default 14 days
    InputTrustingPeriod(String),
    /// RPC timeout: Duration, default 3 secs
    InputMaxClockDrift(String),
    /// rpc_timeout: Duration
    InputRpcTimeout(String),
    /// Diversifier used in transactions for chain
    InputDiversifier(String),
    /// Port ID used to create connection with chain: PortId
    InputPortId(String),
}

pub fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

pub fn empty_message<'a>(message: &str) -> Element<'a, Message> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}
