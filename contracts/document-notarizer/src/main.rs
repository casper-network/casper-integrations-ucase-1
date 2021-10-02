#![no_main]
#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    collections::BTreeSet,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use associate_type::AssociateType;
use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPoints, Group, Key, Parameter, PublicKey, RuntimeArgs, URef,
};
use documents::Documents;
use error::NotarizerError;

pub mod associate_type;
pub mod documents;
pub mod error;

#[no_mangle]
pub extern "C" fn constructor() {
    Documents::init();
}

#[no_mangle]
pub extern "C" fn notarize_document() {
    if can_have_access() {
        let document_hash: Key = runtime::get_named_arg("document_hash");
        let documents_meta: Option<Vec<String>> = runtime::get_named_arg("document_meta");
        Documents::open().notarize_document(document_hash, documents_meta);
    } else {
        revert(NotarizerError::NotarizeDocumentNoAccess)
    }
}

#[no_mangle]
pub extern "C" fn get_document_meta() {
    if can_have_access() {
        let document_hash: Key = runtime::get_named_arg("document_hash");
        runtime::ret(
            CLValue::from_t(Documents::open().get_document_meta(&document_hash)).unwrap_or_revert(),
        )
    } else {
        revert(NotarizerError::GetDocumentMetaNoAccessRights)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _access) = storage::create_contract_package_at_hash();
    let entry_points = {
        let mut eps = EntryPoints::new();
        eps.add_entry_point(EntryPoint::new(
            "constructor",
            vec![],
            CLType::Unit,
            EntryPointAccess::Groups(vec![Group::new("constructor")]),
            casper_types::EntryPointType::Contract,
        ));
        eps.add_entry_point(EntryPoint::new(
            "notarize_document",
            vec![
                Parameter::new("document_hash", CLType::Key),
                Parameter::new("document_meta", CLType::List(Box::new(CLType::String))),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps.add_entry_point(EntryPoint::new(
            "get_document_meta",
            vec![Parameter::new("document_hash", CLType::Key)],
            CLType::List(Box::new(CLType::String)),
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps
    };

    let named_keys = {
        let registrar_hash: Key = Key::Hash(
            runtime::get_named_arg::<Key>("registrar")
                .into_hash()
                .unwrap_or_default(),
        );
        let mut nk = NamedKeys::new();
        nk.insert("registrar".to_string(), registrar_hash);
        nk
    };
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    run_constructor(contract_package_hash);
    // wrap the contract hash so that it can be reached from the test environment
    runtime::put_key("document_notarizer_contract_hash", contract_hash.into());
    runtime::put_key(
        "document_notarizer_contract_hash_wrapped",
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        "document_notarizer_contract_package_hash",
        contract_package_hash.into(),
    );
    runtime::put_key(
        "document_notarizer_contract_package_hash_wrapped",
        storage::new_uref(contract_package_hash).into(),
    );
}

fn run_constructor(contract_package_hash: ContractPackageHash) {
    let constructor_access: URef = storage::create_contract_user_group(
        contract_package_hash,
        "constructor",
        1,
        Default::default(),
    )
    .unwrap_or_revert_with(NotarizerError::ConstructorGroupCreation)
    .pop()
    .unwrap_or_revert_with(NotarizerError::ConstructorGroupPopEmpty);
    let _: () = runtime::call_versioned_contract(
        contract_package_hash,
        None,
        "constructor",
        runtime_args! {},
    );
    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(contract_package_hash, "constructor", urefs)
        .unwrap_or_revert_with(NotarizerError::ConstructorGroupRemove);
}

fn get_caller_association() -> AssociateType {
    let fetched = runtime::call_versioned_contract::<u32>(
        get_registrar(),
        None,
        "get_caller_association",
        runtime_args! {},
    );
    AssociateType::from(fetched)
}

fn _get_associate_type(assoc_pubkey: PublicKey) -> AssociateType {
    let fetched = runtime::call_versioned_contract::<u32>(
        get_registrar(),
        None,
        "get_associate_type",
        runtime_args! {"associate_public_key" => assoc_pubkey},
    );
    AssociateType::from(fetched)
}

fn can_have_access() -> bool {
    get_caller_association() as u32 != 0
}

pub fn get_registrar() -> ContractPackageHash {
    let key = runtime::get_key("registrar")
        .unwrap_or_revert_with(NotarizerError::GetRegistrarKeyNotFound);
    match key {
        Key::URef(uref) => storage::read(uref)
            .unwrap_or_revert_with(NotarizerError::GetRegistrarReadError)
            .unwrap_or_revert_with(NotarizerError::GetRegistrarReadNotFound),
        Key::Hash(hash) => ContractPackageHash::from(hash),
        _ => revert(NotarizerError::GetRegistrarKeyNotURef),
    }
}
