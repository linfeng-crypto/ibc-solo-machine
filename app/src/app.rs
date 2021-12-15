use crate::message::{self, empty_message, loading_message, ChainMessage, Message};
use crate::pages::add_chain_page::AddChainAdvancedPage;
use crate::pages::main_page::Filter;
use crate::pages::main_page::MainPage;
use crate::pages::Page;
use crate::state::{SavedState, State};

use iced::{
    Application, Clipboard, Column, Command, Container, Element, HorizontalAlignment, Length,
    Scrollable, Text,
};
use solo_machine::signer::SignerRegistrar;
use solo_machine_core::service::IbcService;
use std::convert::TryFrom;
use tokio::runtime::Runtime;

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
                        current_page: Page::MainPage,
                        main_page,
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
                        if let Page::MainPage = state.current_page {
                            if let Some(chain) = state.main_page.chains.get_mut(index) {
                                match chain_message {
                                    message::ChainMessage::DoAction(chain_id) => {
                                        log::info!("do action on chain_id: {:?}", chain_id);
                                        // solo-machine ibc connect chain_id.id
                                        let run_time = Runtime::new().unwrap();
                                        let _: anyhow::Result<()> = run_time.block_on(async move {
                                            let db_pool = SavedState::db_pool().await?;
                                            let ibc_service = IbcService::new(db_pool);
                                            let memo = "".to_string();
                                            let page = AddChainAdvancedPage::default();
                                            log::info!("page sign path: {:?}", page.signer_path);
                                            let signer =
                                                SignerRegistrar::try_from(page.signer_path)
                                                    .unwrap()
                                                    .unwrap()?;
                                            log::info!("get signer ok");
                                            ibc_service
                                                .connect(signer, chain_id, None, memo, false)
                                                .await?;
                                            Ok(())
                                        });
                                    }
                                    ChainMessage::DoClose(chain_id) => {
                                        log::info!("do close on chain_id: {:?}", chain_id);
                                        let run_time = Runtime::new().unwrap();
                                        let _: anyhow::Result<()> = run_time.block_on(async move {
                                            let db_pool = SavedState::db_pool().await?;
                                            let ibc_service = IbcService::new(db_pool);
                                            let memo = "".to_string();
                                            let page = AddChainAdvancedPage::default();
                                            log::info!("page sign path: {:?}", page.signer_path);
                                            let signer =
                                                SignerRegistrar::try_from(page.signer_path)
                                                    .unwrap()
                                                    .unwrap()?;
                                            log::info!("get signer ok");
                                            ibc_service
                                                .close_channel(signer, &chain_id, None, memo)
                                                .await?;
                                            Ok(())
                                        });
                                    }
                                    ChainMessage::ShowDetailInfo(_) => {
                                        log::info!("show chain details");
                                        // todo!("show the chain details");
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Message::NewChainMessage(chain_message) => {
                        if let Page::AddChainInput = state.current_page {
                            state.add_chain_page.basic.update(chain_message).unwrap();
                        }
                    }
                    Message::FilterChanged(filter) => match filter {
                        Filter::New => {
                            state.main_page.filter = filter;
                            state.current_page = Page::AddChainInput;
                        }
                        _ => {
                            state.main_page.filter = filter;
                            state.current_page = Page::MainPage;
                            return Command::perform(SavedState::load(), Message::Loaded);
                        }
                    },
                    Message::PressNewConnection(chai_name) => {}
                    Message::PressMintChain => {}
                    Message::PressBurnToken => {}
                    Message::Exit => {}
                    Message::Saved => {
                        saved = true;
                        state.saving = false;
                    }
                    Message::Loaded(Ok(saved_state)) => {
                        state.main_page.chains = saved_state.chains;
                    }
                    m => {
                        log::info!("get other message in loaded: {:?}", m);
                    }
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
                current_page,
                main_page,
                add_chain_page,
            }) => {
                let title = Text::new(APP_NAME)
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);

                let controller = main_page
                    .controller
                    .view(&main_page.chains, main_page.filter);

                let page_content = match current_page {
                    Page::MainPage => {
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
                                Filter::Closed => empty_message("All your chains are connected"),
                                Filter::New => empty_message("All your chains are connected"),
                            }
                        };
                        chains
                    }
                    Page::AddChainInput => add_chain_page
                        .basic
                        .view()
                        .into_iter()
                        .fold(Column::new().spacing(20), |column, e| {
                            column.push(e.map(move |msg| Message::NewChainMessage(msg)))
                        })
                        .into(),
                    _ => empty_message("All your chains are connected"),
                };
                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(controller)
                    .push(page_content);

                Scrollable::new(&mut main_page.scroll)
                    .padding(40)
                    .push(Container::new(content).width(Length::Fill).center_x())
                    .into()
            }
        }
    }
}
