//! Support library for Crypto/Cosmos Ledger Nano S/X apps
use crate::{LedgerApp, LedgerTrait, PubkeyAddress};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ledger_transport::{APDUCommand, APDUErrorCodes, APDUTransport};
use ledger_zondax_generic::{
    map_apdu_error_description, AppInfo, ChunkPayloadType, DeviceInfo, LedgerAppError, Version,
};
use zx_bip44::BIP44Path;

pub struct LeaderHid<T> {
    apdu_transport: APDUTransport,
    app: T,
}

impl<T> LeaderHid<T> {
    pub fn new(app: T) -> Result<Self> {
        let wrapper = ledger::TransportNativeHID::new().map_err(|e| {
            anyhow!("can't find ledger device: {:?}, see more: https://support.ledger.com/hc/en-us/articles/115005165269-Fix-connection-issues", e)
        })?;
        let apdu_transport = APDUTransport {
            transport_wrapper: Box::new(wrapper),
        };
        Ok(Self {
            apdu_transport,
            app,
        })
    }
}

#[async_trait]
impl<T: LedgerApp + Send + Sync> LedgerTrait for LeaderHid<T> {
    async fn get_version(&self) -> Result<Version, LedgerAppError> {
        ledger_zondax_generic::get_version(self.app.cla(), &self.apdu_transport).await
    }

    async fn get_app_info(&self) -> Result<AppInfo, LedgerAppError> {
        ledger_zondax_generic::get_app_info(&self.apdu_transport).await
    }

    async fn get_device_info(&self) -> Result<DeviceInfo, LedgerAppError> {
        ledger_zondax_generic::get_device_info(&self.apdu_transport).await
    }

    async fn get_pubkey_address(
        &self,
        acc_address_prefix: &str,
        path: &BIP44Path,
        require_confirmation: bool,
    ) -> Result<PubkeyAddress, LedgerAppError> {
        let mut data = vec![];
        let acc_address_prefix_len = acc_address_prefix.as_bytes().len();
        data.push(acc_address_prefix_len as u8);
        let mut acc_address_prefix_raw = acc_address_prefix.as_bytes().to_vec();
        data.append(&mut acc_address_prefix_raw);
        let mut serialized_path = path.serialize();
        data.append(&mut serialized_path);
        let p1 = if require_confirmation { 1 } else { 0 };

        let command = APDUCommand {
            cla: self.app.cla(),
            ins: self.app.ins_get_addr_secp256k1(),
            p1,
            p2: 0x00,
            data,
        };

        log::debug!("apdu command: {:?}", command);

        let response = self.apdu_transport.exchange(&command).await?;
        if response.retcode != 0x9000 {
            return Err(LedgerAppError::AppSpecific(
                response.retcode,
                map_apdu_error_description(response.retcode).to_string(),
            ));
        }

        log::debug!("Received response {}", response.data.len());
        let pubkey_len = self.app.pubkey_len();
        if response.data.len() < pubkey_len {
            return Err(LedgerAppError::InvalidPK);
        }

        let mut pubkey_address = PubkeyAddress {
            raw_public_key: vec![],
            address: "".to_string(),
        };

        pubkey_address.raw_public_key = response.data[..pubkey_len].to_vec();
        pubkey_address.address = String::from_utf8(response.data[pubkey_len..].to_vec())
            .map_err(|_e| LedgerAppError::Utf8)?
            .to_owned();
        log::debug!("address: {:?}", pubkey_address.address);
        Ok(pubkey_address)
    }

    async fn sign_message(
        &self,
        path: &BIP44Path,
        message: &[u8],
    ) -> Result<Vec<u8>, LedgerAppError> {
        let serialized_path = path.serialize();
        let start_command = APDUCommand {
            cla: self.app.cla(),
            ins: self.app.ins_sign_secp256k1(),
            p1: ChunkPayloadType::Init as u8,
            p2: 0x00,
            data: serialized_path,
        };

        log::debug!("sign ->");
        let response =
            ledger_zondax_generic::send_chunks(&self.apdu_transport, &start_command, message)
                .await?;
        log::debug!("sign OK");

        if response.data.is_empty() && response.retcode == APDUErrorCodes::NoError as u16 {
            return Err(LedgerAppError::NoSignature);
        }
        let signature_len = self.app.signature_len();

        // Last response should contain the answer
        if response.data.len() < signature_len {
            return Err(LedgerAppError::InvalidSignature);
        }
        log::debug!("sign response: {:?}", response.data);
        let sig = response.data[..signature_len].to_vec();
        Ok(sig)
    }
}
