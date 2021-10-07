#![no_main]
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use casper_contract::contract_api::runtime::{self, revert};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs};

#[no_mangle]
pub extern "C" fn call() {
    let document_hash: Key = runtime::get_named_arg("document_hash");
    let expected_result: Option<Vec<String>> = runtime::get_named_arg("expected_result");
    let notarizer_contract_hash = runtime::get_named_arg("contract_hash");
    let ret: Option<Vec<String>> = runtime::call_versioned_contract(
        notarizer_contract_hash,
        None,
        "get_document_meta",
        runtime_args! {"document_hash"=>document_hash},
    );
    if ret != expected_result {
        revert(ApiError::User(900));
    }
}
