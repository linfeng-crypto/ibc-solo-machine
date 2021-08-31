use anyhow::{Context, Result};
use async_trait::async_trait;
use solo_machine_core::cosmos::crypto::PublicKey;
use solo_machine_core::signer::{AddressAlgo, LedgerCurrency, Message};
use solo_machine_core::{Signer, ToPublicKey};
use std::sync::Arc;
use zx_bip44::BIP44Path;

use crate::apps::{CosmosApp, CryptoApp};
use crate::ledger_hid::LeaderHid;
use crate::LedgerTrait;

const DEFAULT_HD_PATH: &str = "m/44'/118'/0'/0/0";
const DEFAULT_ACCOUNT_PREFIX: &str = "cosmos";
const DEFAULT_ADDRESS_ALGO: &str = "secp256k1";
const DEFAULT_LEDGER_CURRENCY: &str = "cosmos";
const DEFAULT_LEDGER_REQUIRE_CONFIRMATION: &str = "false";

pub struct LedgerSigner {
    /// ledger
    pub ledger: Box<dyn LedgerTrait>,
    /// chain path
    pub hd_path: Arc<BIP44Path>,
    /// Bech 32 prefix
    pub account_prefix: String,
    /// Algorithm used for address generation
    pub algo: AddressAlgo,
    /// confirmation on ledger or not
    pub require_confirmation: bool,
}

#[async_trait]
impl ToPublicKey for LedgerSigner {
    async fn to_public_key(&self) -> Result<PublicKey> {
        let pubkey_address = self
            .ledger
            .get_pubkey_address(
                &self.account_prefix,
                &self.hd_path,
                self.require_confirmation,
            )
            .await?;
        let verify_key =
            k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_address.raw_public_key)?;
        match self.algo {
            AddressAlgo::Secp256k1 => Ok(PublicKey::Secp256k1(verify_key)),
            #[cfg(feature = "ethermint")]
            AddressAlgo::EthSecp256k1 => Ok(PublicKey::EthSecp256k1(verifying_key)),
        }
    }

    fn get_account_prefix(&self) -> &str {
        &self.account_prefix
    }

    async fn to_account_address(&self) -> Result<String> {
        self.to_public_key()
            .await?
            .account_address(self.get_account_prefix())
    }
}

#[async_trait]
impl Signer for LedgerSigner {
    async fn sign(&self, _request_id: Option<&str>, message: Message<'_>) -> Result<Vec<u8>> {
        let data = message.as_ref();
        let result = self.ledger.sign_message(&self.hd_path, data).await?;
        Ok(result)
    }
}

fn get_env(key: &str) -> Result<String> {
    std::env::var(key).context(format!(
        "`{}` environment variable is required for mnemonic signer",
        key
    ))
}

impl LedgerSigner {
    /// create a new LedgerService
    pub fn new(
        ledger: Box<dyn LedgerTrait>,
        hd_path: &str,
        account_prefix: String,
        algo: AddressAlgo,
        require_confirmation: bool,
    ) -> Result<Self> {
        let path = BIP44Path::from_string(hd_path).context("input invalid hd path")?;
        Ok(Self {
            ledger,
            hd_path: Arc::new(path),
            account_prefix,
            algo,
            require_confirmation,
        })
    }

    pub fn from_env() -> Result<Self> {
        let ledger_currency = get_env("LEDGER_CURRENCY")
            .unwrap_or_else(|_| DEFAULT_LEDGER_CURRENCY.to_string())
            .parse()?;
        let hd_path = get_env("SOLO_HD_PATH").unwrap_or_else(|_| DEFAULT_HD_PATH.to_string());
        let account_prefix =
            get_env("SOLO_ACCOUNT_PREFIX").unwrap_or_else(|_| DEFAULT_ACCOUNT_PREFIX.to_string());
        let require_confirmation = get_env("LEDGER_REQUIRE_CONFIRMATION")
            .unwrap_or_else(|_| DEFAULT_LEDGER_REQUIRE_CONFIRMATION.to_string());
        let require_confirmation = match require_confirmation.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => false,
        };

        let algo = get_env("SOLO_ADDRESS_ALGO")
            .unwrap_or_else(|_| DEFAULT_ADDRESS_ALGO.to_string())
            .parse()?;
        let ledger = match ledger_currency {
            LedgerCurrency::CryptoCom => {
                let app = CryptoApp;
                let ledger_hid = LeaderHid::new(app)?;
                Box::new(ledger_hid) as Box<dyn LedgerTrait>
            }
            LedgerCurrency::Cosmos => {
                let app = CosmosApp;
                let ledger_hid = LeaderHid::new(app)?;
                Box::new(ledger_hid) as Box<dyn LedgerTrait>
            }
            #[cfg(feature = "ethermint")]
            LedgerCurrency::Ethermint => {
                todo!("create Ethereum app")
            }
        };
        let signer =
            LedgerSigner::new(ledger, &hd_path, account_prefix, algo, require_confirmation)?;
        Ok(signer)
    }
}
