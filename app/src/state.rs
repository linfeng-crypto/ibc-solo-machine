use anyhow::Result;
use solo_machine_core::model::get_chains;
use solo_machine_core::{connect_db, init_db, run_migrations, DbPool};
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

use crate::chain::Chain;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::pages::add_chain_page::AddChainPage;
use crate::pages::main_page::MainPage;
use crate::pages::Page;

#[derive(Default, Debug)]
pub struct State {
    pub saving: bool,
    pub dirty: bool,
    pub current_page: Page,
    pub main_page: MainPage,
    pub add_chain_page: AddChainPage,
}

/// the saved info in file and db
#[derive(Clone, Debug)]
pub struct SavedState {
    /// the app config info
    pub app_config: AppConfig,
    /// the chain info
    pub chains: Vec<Chain>,
}

impl SavedState {
    /// Linux:   ~/.config/solo-machine
    /// Windows: C:\Users\Alice\AppData\Roaming\crypto\solo-machine
    /// macOS:   ~/Library/Application Support/com.crypto.solo-machine
    fn data_path() -> PathBuf {
        // let path = if let Some(project_dirs) =
        //     directories_next::ProjectDirs::from("com", "crypto", "solo-machine")
        // {
        //     project_dirs.data_dir().into()
        // } else {
        //     std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        // };
        // path
        use std::str::FromStr;
        PathBuf::from_str("./").unwrap()
    }

    /// set the config path
    fn config_path() -> PathBuf {
        let mut path = Self::data_path();
        path.push("solomachine.json");
        path
    }

    /// set the database path
    fn db_path() -> PathBuf {
        let mut path = Self::data_path();
        path.push("solomachine.db");
        path
    }

    /// get the saved state
    pub async fn load_config() -> Result<AppConfig> {
        let mut contents = String::new();
        let config_file = Self::config_path();
        if config_file.exists() {
            let mut file = tokio::fs::File::open(config_file).await?;
            file.read_to_string(&mut contents).await?;
            let s = serde_json::from_str(&contents)?;
            Ok(s)
        } else {
            Ok(AppConfig {})
        }
    }

    pub async fn db_pool() -> Result<DbPool> {
        let db_path = Self::db_path();
        let db_uri = format!("{}", db_path.display());
        let db_pool = connect_db(&db_uri).await?;
        Ok(db_pool)
    }

    /// query chain id list from local sqlite db file
    // TODO: add page and item limit
    pub async fn get_chains() -> Result<Vec<Chain>> {
        let db_pool = Self::db_pool().await?;
        let chain_ids = get_chains(&db_pool)
            .await?
            .into_iter()
            .map(|c| Chain::new(c))
            .collect();
        Ok(chain_ids)
    }

    async fn init() -> Result<()> {
        let data_path = Self::data_path();
        if !data_path.exists() {
            std::fs::create_dir_all(data_path).unwrap();
        }
        let db_path = Self::db_path();
        if db_path.exists() {
            return Ok(());
        }
        let db_uri = format!("{}", db_path.display());
        init_db(&db_uri).await?;
        let db_pool = connect_db(&db_uri).await?;
        run_migrations(&db_pool).await?;
        log::info!("init data path success");
        Ok(())
    }

    pub async fn load() -> std::result::Result<Self, AppError> {
        log::info!("loading data from db");
        Self::init().await.map_err(|e| {
            log::error!("{:?}", e);
            AppError::DbError
        })?;
        let app_config = Self::load_config()
            .await
            .map_err(|_e| AppError::FileError)?;
        let chains = Self::get_chains().await.map_err(|_e| AppError::DbError)?;
        log::info!("get {} chains from db", chains.len());
        Ok(Self { app_config, chains })
    }
}
