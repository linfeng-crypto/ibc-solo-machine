use crate::ledger_signer::LedgerSigner;
use async_trait::async_trait;
use ledger_zondax_generic::{AppInfo, DeviceInfo, LedgerAppError, Version};
use solo_machine_core::signer::SignerRegistrar;
use std::sync::Arc;
use zx_bip44::BIP44Path;

pub mod apps;
pub mod ledger_hid;
pub mod ledger_signer;

#[derive(Clone, Debug)]
pub struct PubkeyAddress {
    /// Public Key
    pub raw_public_key: Vec<u8>,
    /// Address (exposed as SS58)
    pub address: String,
}

#[async_trait]
pub trait LedgerTrait: Sync + Send {
    /// Retrieve the app version
    async fn get_version(&self) -> Result<Version, LedgerAppError>;

    /// Retrieve the app info
    async fn get_app_info(&self) -> Result<AppInfo, LedgerAppError>;

    /// Retrieve the device info
    async fn get_device_info(&self) -> Result<DeviceInfo, LedgerAppError>;

    /// Retrieves the public key and address
    async fn get_pubkey_address(
        &self,
        acc_address_prefix: &str,
        path: &BIP44Path,
        require_confirmation: bool,
    ) -> Result<PubkeyAddress, LedgerAppError>;

    /// Sign a transaction
    async fn sign_message(
        &self,
        path: &BIP44Path,
        message: &[u8],
    ) -> Result<Vec<u8>, LedgerAppError>;
}

pub trait LedgerApp {
    fn cla(&self) -> u8;
    fn ins_get_addr_secp256k1(&self) -> u8;
    fn ins_sign_secp256k1(&self) -> u8;
    fn pubkey_len(&self) -> usize;
    fn signature_len(&self) -> usize;
}

#[no_mangle]
pub fn register_signer(registrar: &mut dyn SignerRegistrar) -> anyhow::Result<()> {
    registrar.register(Arc::new(LedgerSigner::from_env()?));
    Ok(())
}
