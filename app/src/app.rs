use crate::message::{self, empty_message, loading_message, AddChainConfig, ChainMessage, Message};
use iced::{
    Application, Clipboard, Column, Command, Container, Element, HorizontalAlignment, Length,
    Scrollable, Text,
};

use crate::pages::main_page::Filter;
use crate::pages::main_page::MainPage;
use crate::pages::Page;
use crate::state::{SavedState, State};

static APP_NAME: &'static str = "IBC Solo Machine";

/// the app
pub enum App {
    Loading,
    Loaded(State),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    /// loading config and chains from local config file and db
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Self::Loading => false,
            Self::Loaded(state) => state.dirty,
        };

        format!("Ibc Solo Machine {}", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match self {
            // when open app first, loading and set the main page
            Self::Loading => match message {
                Message::Loaded(Ok(saved_state)) => {
                    let mut main_page = MainPage::default();
                    main_page.chains = saved_state.chains;
                    *self = Self::Loaded(State {
                        page: Page::MainPage(main_page),
                        ..Default::default()
                    });
                }
                Message::Loaded(Err(_)) => {
                    *self = Self::Loaded(State::default());
                }
                _ => {}
            },
            // when input or press button in the pages
            Self::Loaded(state) => {
                let mut saved = false;
                match message {
                    Message::ChainMessage(index, chain_message) => {
                        if let Page::MainPage(main_page) = &mut state.page {
                            if let Some(chain) = main_page.chains.get_mut(index) {
                                match chain_message {
                                    message::ChainMessage::DoAction(_) => {
                                        todo!("connection again");
                                        chain.update(ChainMessage::SetActive);
                                    }
                                    ChainMessage::DoDisconnect(_) => {
                                        todo!("disconnection again");
                                        chain.update(message::ChainMessage::SetDisconnected);
                                    }
                                    ChainMessage::ShowDetailInfo(_) => {
                                        todo!("show the chain details");
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Message::InputAddChainConfig(input) => {
                        if let Page::AddChainInput(page) = &mut state.page {
                            match input {
                                AddChainConfig::InputGrpcAddress(s) => {
                                    page.basic.input_grpc_address = s;
                                }
                                AddChainConfig::InputRpcAddress(s) => {
                                    page.basic.input_rpc_address = s;
                                }
                            }
                        }
                    }
                    Message::InputAddChainConfigAdcanced(input) => {
                        // TODO: set the input to the advanced config and store into the file
                    }
                    Message::PressAddChain => {
                        // TODO: add chain
                    }
                    Message::FilterChanged(filter) => {
                        if let Page::MainPage(page) = &mut state.page {
                            page.filter = filter;
                        }
                    }
                    Message::PressNewConnection(chai_name) => {}
                    Message::PressMintChain => {}
                    Message::PressBurnToken => {}
                    Message::Exit => {}
                    Message::Saved => {
                        saved = true;
                        state.saving = false;
                    }
                    _ => {}
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Self::Loading => loading_message(),
            Self::Loaded(State {
                saving,
                dirty,
                page,
            }) => match page {
                Page::MainPage(main_page) => {
                    let title = Text::new(APP_NAME)
                        .width(Length::Fill)
                        .size(100)
                        .color([0.5, 0.5, 0.5])
                        .horizontal_alignment(HorizontalAlignment::Center);

                    let controller = main_page
                        .controller
                        .view(&main_page.chains, main_page.filter);
                    let filtered_chains = main_page
                        .chains
                        .iter()
                        .filter(|chain| main_page.filter.matches(chain));

                    let chains: Element<_> = if filtered_chains.count() > 0 {
                        main_page
                            .chains
                            .iter_mut()
                            .enumerate()
                            .filter(|(_, chain)| main_page.filter.matches(chain))
                            .fold(Column::new().spacing(20), |column, (i, chain)| {
                                column.push(
                                    chain
                                        .view()
                                        .map(move |message| Message::ChainMessage(i, message)),
                                )
                            })
                            .into()
                    } else {
                        match main_page.filter {
                            Filter::All => empty_message("You have not connected chain"),
                            Filter::Active => empty_message("You have no active chain"),
                            Filter::DisConnected => empty_message("All your chains connected"),
                        }
                    };

                    let content = Column::new()
                        .max_width(800)
                        .spacing(20)
                        .push(title)
                        .push(controller)
                        .push(chains);

                    Scrollable::new(&mut main_page.scroll)
                        .padding(40)
                        .push(Container::new(content).width(Length::Fill).center_x())
                        .into()
                }
                _ => {
                    todo!("add other page")
                }
            },
        }
    }
}
