use anyhow::Result;
use solo_machine_core::connect_db;
use solo_machine_core::model::get_chains;
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::AsyncReadExt;

use crate::chain::Chain;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::pages::Page;

#[derive(Default, Debug)]
pub struct State {
    pub saving: bool,
    pub dirty: bool,
    pub page: Page,
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
        let path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("com", "crypto", "solo-machine")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path
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
        let mut file = tokio::fs::File::open(Self::config_path()).await?;
        file.read_to_string(&mut contents).await?;
        let s = serde_json::from_str(&contents)?;
        Ok(s)
    }

    /// query chain id list from local sqlite db file
    // TODO: add page and item limit
    pub async fn get_chains() -> Result<Vec<Chain>> {
        let db_path = Self::db_path();
        let db_uri = format!("{}", db_path.display());
        let db_pool = connect_db(&db_uri).await?;
        let chain_ids = get_chains(&db_pool)
            .await?
            .into_iter()
            .map(|c| Chain::new(c))
            .collect();
        Ok(chain_ids)
    }

    pub async fn load() -> std::result::Result<Self, AppError> {
        // TODO: if db file is not exist, init the db
        let app_config = Self::load_config()
            .await
            .map_err(|_e| AppError::FileError)?;
        let chains = Self::get_chains().await.map_err(|_e| AppError::DbError)?;
        Ok(Self { app_config, chains })
    }
}
