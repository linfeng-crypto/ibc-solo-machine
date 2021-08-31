//! Utilities for signing transactions
use std::{fmt, str::FromStr, sync::Arc};

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;

use crate::cosmos::crypto::PublicKey;

#[derive(Debug, Clone, Copy)]
/// Supported algorithms for address generation
pub enum AddressAlgo {
    /// Secp256k1 (tendermint)
    Secp256k1,
    #[cfg(feature = "ethermint")]
    /// EthSecp256k1 (ethermint)
    EthSecp256k1,
}

impl fmt::Display for AddressAlgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Secp256k1 => write!(f, "secp256k1"),
            #[cfg(feature = "ethermint")]
            Self::EthSecp256k1 => write!(f, "eth-secp256k1"),
        }
    }
}

impl FromStr for AddressAlgo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "secp256k1" => Ok(Self::Secp256k1),
            #[cfg(feature = "ethermint")]
            "eth-secp256k1" => Ok(Self::EthSecp256k1),
            _ => Err(anyhow!("invalid address generation algorithm: {}", s)),
        }
    }
}

/// Type of message given to a signer
#[derive(Debug)]
pub enum Message<'a> {
    /// [cosmos_sdk_proto::ibc::lightclients::solomachine::v1::SignBytes]
    SignBytes(&'a [u8]),
    /// [cosmos_sdk_proto::cosmos::tx::v1beta1::SignDoc]
    SignDoc(&'a [u8]),
}

impl<'a> Message<'a> {
    /// Returns the message type of current message
    pub fn message_type(&self) -> &'static str {
        match self {
            Self::SignBytes(_) => "sign-bytes",
            Self::SignDoc(_) => "sign-doc",
        }
    }
}

impl AsRef<[u8]> for Message<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::SignBytes(bytes) => bytes,
            Self::SignDoc(bytes) => bytes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Ledger supported currencies
pub enum LedgerCurrency {
    /// crypto.com coin
    CryptoCom,
    /// Cosmos coin
    Cosmos,
    #[cfg(feature = "ethermint")]
    /// ethermint coin
    Ethermint,
}

impl FromStr for LedgerCurrency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "crypto-com" => Ok(Self::CryptoCom),
            "cosmos" => Ok(Self::Cosmos),
            #[cfg(feature = "ethermint")]
            "ethermint" => Ok(Self::Ethermint),
            _ => Err(anyhow!(
                "invalid currency {}, only support crypto-com|cosmos|ethermint",
                s
            )),
        }
    }
}

/// This trait must be implemented by all the public key providers (e.g. mnemonic, ledger, etc.)
#[async_trait]
pub trait ToPublicKey {
    /// Returns public key of signer
    async fn to_public_key(&self) -> Result<PublicKey>;

    /// Returns account prefix for computing bech32 addresses
    fn get_account_prefix(&self) -> &str;

    /// Returns accounts address for this signer for given prefix
    async fn to_account_address(&self) -> Result<String>;
}

#[async_trait]
impl<T: ToPublicKey + Send + Sync> ToPublicKey for &T {
    async fn to_public_key(&self) -> Result<PublicKey> {
        (*self).to_public_key().await
    }

    fn get_account_prefix(&self) -> &str {
        (*self).get_account_prefix()
    }

    async fn to_account_address(&self) -> Result<String> {
        (*self).to_account_address().await
    }
}

#[async_trait]
impl<T: ToPublicKey + ?Sized + Sync + Send> ToPublicKey for Arc<T> {
    async fn to_public_key(&self) -> Result<PublicKey> {
        (**self).to_public_key().await
    }

    fn get_account_prefix(&self) -> &str {
        (**self).get_account_prefix()
    }

    async fn to_account_address(&self) -> Result<String> {
        (**self).to_account_address().await
    }
}

/// This trait must be implemented by all the transaction signers (e.g. mnemonic, ledger, etc.)
#[async_trait]
pub trait Signer: ToPublicKey + Send + Sync {
    /// Signs the given message
    async fn sign(&self, request_id: Option<&str>, message: Message<'_>) -> Result<Vec<u8>>;
}

#[async_trait]
impl<T: Signer> Signer for &T {
    async fn sign(&self, request_id: Option<&str>, message: Message<'_>) -> Result<Vec<u8>> {
        (*self).sign(request_id, message).await
    }
}

#[async_trait]
impl<T: Signer + ?Sized> Signer for Arc<T> {
    async fn sign(&self, request_id: Option<&str>, message: Message<'_>) -> Result<Vec<u8>> {
        (**self).sign(request_id, message).await
    }
}

/// Trait to register a signer
pub trait SignerRegistrar {
    /// Registers a new signer
    fn register(&mut self, signer: Arc<dyn Signer>);
}
