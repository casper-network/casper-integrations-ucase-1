#![no_std]
#![allow(unused)]
extern crate alloc;
use core::convert::TryInto;

use super::error::NotarizerError;
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
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, Key, URef,
};

pub struct Documents {
    document: URef,
    submitter: URef,
}

impl Documents {
    pub fn init() {
        storage::new_dictionary("documents")
            .unwrap_or_revert_with(NotarizerError::InitDocumentsDictionaryURef);
        storage::new_dictionary("submitter")
            .unwrap_or_revert_with(NotarizerError::InitSubmitterDictionaryURef);
    }

    pub fn open() -> Self {
        let document_key: Key = runtime::get_key("documents")
            .unwrap_or_revert_with(NotarizerError::OpenDocumentsDictionaryKey);
        let document: URef = *document_key
            .as_uref()
            .unwrap_or_revert_with(NotarizerError::OpenDocumentsDictionaryURef);
        let submitter_key: Key = runtime::get_key("submitter")
            .unwrap_or_revert_with(NotarizerError::OpenSubmitterDictionaryKey);
        let submitter: URef = *submitter_key
            .as_uref()
            .unwrap_or_revert_with(NotarizerError::OpenSubmitterDictionaryURef);
        Self {
            document,
            submitter,
        }
    }

    pub fn get_document_meta(&self, document_hash: &Key) -> Option<Vec<String>> {
        match document_hash {
            Key::Hash(hash) => storage::dictionary_get(self.document, &base64::encode(hash))
                .unwrap_or_revert()
                .unwrap_or_default(),
            _ => revert(NotarizerError::GetDocumentMetaKeyNotHash),
        }
    }

    pub fn notarize_document(&self, document_hash: Key, document_meta: Option<Vec<String>>) {
        match document_hash {
            Key::Hash(hash) => {
                storage::dictionary_put(self.document, &base64::encode(hash), document_meta)
            }
            _ => revert(NotarizerError::NotarizeDocumentKeyNotHash),
        }
    }
}
