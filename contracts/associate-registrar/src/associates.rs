#![no_std]
#![allow(unused)]
extern crate alloc;
use super::error::RegistrarError;
use crate::alloc::borrow::ToOwned;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    ApiError, AsymmetricType, CLTyped, Key, PublicKey, URef,
};

#[derive(Clone, Copy)]
pub enum AssociateType {
    NotMember = 0,
    Partner = 1,
    Judge = 2,
    Lawyer = 3,
    Clerk = 4,
}

impl From<AssociateType> for u32 {
    fn from(fin: AssociateType) -> u32 {
        fin as u32
    }
}

impl From<u32> for AssociateType {
    fn from(u: u32) -> AssociateType {
        match u {
            0 => AssociateType::NotMember,
            1 => AssociateType::Partner,
            2 => AssociateType::Judge,
            3 => AssociateType::Lawyer,
            4 => AssociateType::Clerk,
            _ => revert(RegistrarError::AssociateTypeNonExistant),
        }
    }
}

pub struct Associates {
    associates: URef,
    public_keys: URef,
}

impl Associates {
    pub fn init(partners: Vec<PublicKey>) {
        let associates = storage::new_dictionary("associates")
            .unwrap_or_revert_with(RegistrarError::InitAssociatesDictionary);
        let public_keys = storage::new_dictionary("public_keys")
            .unwrap_or_revert_with(RegistrarError::InitPublicKeyDictionary);
        let associates_dicts = Self {
            associates,
            public_keys,
        };
        for p in partners {
            associates_dicts.store_associate(&p, &AssociateType::Partner)
        }
    }

    pub fn open() -> Self {
        let associates_key: Key = runtime::get_key("associates")
            .unwrap_or_revert_with(RegistrarError::OpenAssociatesDictinonaryKey);
        let associates: URef = *associates_key
            .as_uref()
            .unwrap_or_revert_with(RegistrarError::OpenAssociatesDictinonaryURef);

        let public_key_key: Key = runtime::get_key("public_keys")
            .unwrap_or_revert_with(RegistrarError::OpenPublicKeyDictinonaryKey);
        let public_keys: URef = *public_key_key
            .as_uref()
            .unwrap_or_revert_with(RegistrarError::OpenPublicKeyDictinonaryURef);
        Self {
            associates,
            public_keys,
        }
    }

    pub fn get_associate_account_by_pubkey(&self, pubkey: &PublicKey) -> AccountHash {
        storage::dictionary_get(self.public_keys, &pubkey.to_string())
            .unwrap_or_revert()
            .unwrap_or_revert()
    }

    pub fn get_associate_type_by_pubkey(&self, pubkey: &PublicKey) -> u32 {
        storage::dictionary_get(self.associates, &pubkey.to_string())
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    pub fn get_associate_pubkey_by_account(&self, account_hash: &AccountHash) -> PublicKey {
        storage::dictionary_get(self.public_keys, &account_hash.to_string())
            .unwrap_or_revert()
            .unwrap_or_revert()
    }

    pub fn get_associate_type_by_account(&self, account_hash: &AccountHash) -> u32 {
        storage::dictionary_get(self.associates, &account_hash.to_string())
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    pub fn store_associate(&self, pubkey: &PublicKey, association_type: &AssociateType) {
        let associate_account: AccountHash = pubkey.to_account_hash();
        storage::dictionary_put(
            self.associates,
            &associate_account.to_string(),
            *association_type as u32,
        );
        storage::dictionary_put(
            self.associates,
            &pubkey.to_string(),
            *association_type as u32,
        );
        storage::dictionary_put(self.public_keys, &pubkey.to_string(), associate_account);
        storage::dictionary_put(
            self.public_keys,
            &associate_account.to_string(),
            pubkey.to_owned(),
        );
    }

    pub fn register(&self, pubkey: &PublicKey, association_type: &AssociateType) {
        if 0 == self.get_associate_type_by_pubkey(pubkey) {
            self.store_associate(pubkey, association_type);
        }
    }

    pub fn unregister(&self, pubkey: &PublicKey) {
        if 0 != self.get_associate_type_by_pubkey(pubkey) {
            let associate_account: AccountHash = pubkey.to_account_hash();
            storage::dictionary_put(
                self.associates,
                &associate_account.to_string(),
                AssociateType::NotMember as u32,
            );
            storage::dictionary_put(
                self.associates,
                &pubkey.to_string(),
                AssociateType::NotMember as u32,
            );
        }
    }
}
