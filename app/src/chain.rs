use crate::message::ChainMessage;
use crate::style::{self, button_icon, style_chain_text, style_input_title};
use iced::{button, Align, Button, Element, Row};
use solo_machine_core::model::Chain as DbChain;

#[derive(Debug, Clone)]
pub struct Chain {
    /// the chain info from db
    pub inner: DbChain,
    /// detail button
    pub button_detail: button::State,
    /// status
    pub status: ChainStatus,
}

/// the chain status
#[derive(Debug, Clone)]
pub enum ChainStatus {
    Active(button::State),
    Closed(button::State),
}

/// return connection status
fn is_connected(chain: &DbChain) -> bool {
    match &chain.connection_details {
        None => false,
        Some(detail) => {
            detail.tendermint_channel_id.is_some() && detail.solo_machine_channel_id.is_some()
        }
    }
}

impl Chain {
    pub fn new(db_chain: DbChain) -> Self {
        let chain_status = if is_connected(&db_chain) {
            ChainStatus::Active(button::State::new())
        } else {
            ChainStatus::Closed(button::State::new())
        };
        Chain {
            inner: db_chain,
            button_detail: button::State::new(),
            status: chain_status,
        }
    }

    pub fn is_active(&self) -> bool {
        is_connected(&self.inner)
    }

    /// when get message, do what the message says
    pub fn update(&mut self, message: ChainMessage) {
        match message {
            ChainMessage::SetActive => {
                self.status = ChainStatus::Closed(button::State::new());
            }
            ChainMessage::SetClosed => {
                self.status = ChainStatus::Active(button::State::new());
            }
            _ => {}
        }
    }

    /// how the chain looks like:
    /// status chain_id  action_button detail_button
    pub fn view(&mut self) -> Element<ChainMessage> {
        let (action_button, status_icon) = match &mut self.status {
            ChainStatus::Closed(s) => {
                let button = Button::new(s, button_icon("connect", 150))
                    .on_press(ChainMessage::DoAction(self.inner.id.clone()))
                    .padding(10)
                    .style(style::Button::Icon);
                let status_icon = button_icon("closed", 150);
                (button, status_icon)
            }
            ChainStatus::Active(s) => {
                let button = Button::new(s, button_icon("close", 150))
                    .on_press(ChainMessage::DoClose(self.inner.id.clone()))
                    .padding(10)
                    .style(style::Button::Icon);
                let status_icon = button_icon("connect", 150);
                (button, status_icon)
            }
        };

        Row::new()
            .spacing(40)
            .align_items(Align::Center)
            .push(style_input_title("chain id: "))
            .push(style_chain_text(&self.inner.id))
            .push(action_button)
            // .push(
            //     Button::new(&mut self.button_detail, style_detail_button())
            //         .on_press(ChainMessage::ShowDetailInfo(self.inner.id.clone()))
            //         .padding(10)
            //         .style(style::Button::Icon),
            // )
            .into()
    }
}
