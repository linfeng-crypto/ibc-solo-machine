// Copyright (c) 2020, Zondax GmbH (licensed under the Apache License Version 2.0)
// Modifications Copyright (c) 2021, Foris Limited (licensed under the Apache License, Version 2.0)

use crate::LedgerApp;

mod cro {
    pub const CLA: u8 = 0x55;
    pub const INS_GET_ADDR_SECP256K1: u8 = 0x04;
    pub const INS_SIGN_SECP256K1: u8 = 0x02;
    // TODO: fix this, use 32/66
    pub const PUBKEY_LEN: usize = 33;
    pub const SIGNATURE_LEN: usize = 65;
}

// TODO: fix the values
mod cosmos {
    pub const CLA: u8 = 0x55;
    pub const INS_GET_ADDR_SECP256K1: u8 = 0x04;
    pub const INS_SIGN_SECP256K1: u8 = 0x02;
    pub const PUBKEY_LEN: usize = 32;
    pub const SIGNATURE_LEN: usize = 65;
}

#[derive(Debug, Clone)]
pub struct CryptoApp;
#[derive(Debug, Clone)]
pub struct CosmosApp;

impl LedgerApp for CryptoApp {
    fn cla(&self) -> u8 {
        cro::CLA
    }

    fn ins_get_addr_secp256k1(&self) -> u8 {
        cro::INS_GET_ADDR_SECP256K1
    }

    fn ins_sign_secp256k1(&self) -> u8 {
        cro::INS_SIGN_SECP256K1
    }

    fn pubkey_len(&self) -> usize {
        cro::PUBKEY_LEN
    }

    fn signature_len(&self) -> usize {
        cro::SIGNATURE_LEN
    }
}

impl LedgerApp for CosmosApp {
    fn cla(&self) -> u8 {
        cosmos::CLA
    }

    fn ins_get_addr_secp256k1(&self) -> u8 {
        cosmos::INS_GET_ADDR_SECP256K1
    }

    fn ins_sign_secp256k1(&self) -> u8 {
        cosmos::INS_SIGN_SECP256K1
    }

    fn pubkey_len(&self) -> usize {
        cosmos::PUBKEY_LEN
    }

    fn signature_len(&self) -> usize {
        cosmos::SIGNATURE_LEN
    }
}
