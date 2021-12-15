pub mod add_chain_page;
pub mod main_page;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    /// the main page
    MainPage,
    /// the add chain page
    AddChainInput,
    /// TODO: other page
    Other,
}

impl Default for Page {
    fn default() -> Self {
        Self::MainPage
    }
}
