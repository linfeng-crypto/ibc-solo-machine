pub mod add_chain_page;
pub mod main_page;

#[derive(Debug, Clone)]
pub enum Page {
    /// the main page
    MainPage(main_page::MainPage),
    /// the add chain page
    AddChainInput(add_chain_page::AddChainPage),
    /// TODO: other page
    Other,
}

impl Default for Page {
    fn default() -> Self {
        let mainpage = main_page::MainPage::default();
        Self::MainPage(mainpage)
    }
}
