use crate::chain::Chain;
use crate::message::Message;
use crate::style;
use iced::{button, scrollable, Align, Button, Length, Row, Text};

#[derive(Debug, Clone, Default)]
pub struct MainPage {
    /// the scroll
    pub scroll: scrollable::State,
    /// filter used to filt the chain types
    pub filter: Filter,
    /// all chains from db
    pub chains: Vec<Chain>,
    /// control which type of chain to show
    pub controller: Controller,
}

/// filt what type of chains to show
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    New,
    All,
    Active,
    Closed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    /// check if chain pass the filter
    pub fn matches(&self, chain: &Chain) -> bool {
        match self {
            Self::All => true,
            Self::Active => chain.is_active(),
            Self::Closed => !chain.is_active(),
            Self::New => false,
        }
    }
}

/// the controller of filter
#[derive(Debug, Default, Clone)]
pub struct Controller {
    new_button: button::State,
    all_button: button::State,
    active_button: button::State,
    closed_button: button::State,
}

impl Controller {
    pub fn view(&mut self, chains: &[Chain], current_filter: Filter) -> Row<Message> {
        let Controller {
            new_button,
            all_button,
            active_button,
            Closed_button,
        } = self;

        let invalid_chains_num = chains.iter().filter(|chain| !chain.is_active()).count();

        let filter_button = |state, label, filter, current_filter| {
            let label = Text::new(label).size(16);
            let button = Button::new(state, label).style(style::Button::Filter {
                selected: filter == current_filter,
            });

            button.on_press(Message::FilterChanged(filter)).padding(8)
        };

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new(&format!(
                    "{} {} Closed",
                    invalid_chains_num,
                    if invalid_chains_num == 1 {
                        "chain"
                    } else {
                        "chains"
                    }
                ))
                .width(Length::Fill)
                .size(16),
            )
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(10)
                    .push(filter_button(
                        new_button,
                        "New",
                        Filter::New,
                        current_filter,
                    ))
                    .push(filter_button(
                        all_button,
                        "All",
                        Filter::All,
                        current_filter,
                    ))
                    .push(filter_button(
                        active_button,
                        "Active",
                        Filter::Active,
                        current_filter,
                    ))
                    .push(filter_button(
                        Closed_button,
                        "Closed",
                        Filter::Closed,
                        current_filter,
                    )),
            )
    }
}
