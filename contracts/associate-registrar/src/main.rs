#![no_main]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, vec, vec::Vec};

use casper_contract::{
    contract_api::{
        runtime::{self},
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPoints, Group, Parameter, PublicKey, RuntimeArgs, URef,
};
pub mod associates;
pub mod error;
use associates::{AssociateType, Associates};
use error::RegistrarError;

#[no_mangle]
pub extern "C" fn constructor() {
    let partners: Vec<PublicKey> = runtime::get_named_arg("partners");
    Associates::init(partners);
}

#[no_mangle]
pub extern "C" fn register() {
    let associate_pubkey: PublicKey = runtime::get_named_arg("associate_public_key");
    let association_type: u32 = runtime::get_named_arg("associate_type");
    Associates::open().register(&associate_pubkey, &AssociateType::from(association_type));
}

#[no_mangle]
pub extern "C" fn unregister() {
    let associate_pubkey: PublicKey = runtime::get_named_arg("associate_public_key");
    Associates::open().unregister(&associate_pubkey);
}

#[no_mangle]
pub extern "C" fn get_caller_association() {
    runtime::ret(
        CLValue::from_t(
            Associates::open().get_associate_type_by_account(&runtime::get_caller()) as u32,
        )
        .unwrap_or_revert(),
    );
}

#[no_mangle]
pub extern "C" fn get_associate_type() {
    runtime::ret(
        CLValue::from_t(
            Associates::open()
                .get_associate_type_by_pubkey(&runtime::get_named_arg("associate_public_key"))
                as u32,
        )
        .unwrap_or_revert(),
    );
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _access) = storage::create_contract_package_at_hash();
    let entry_points = get_entry_points();

    // Store contract and data related to it
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, NamedKeys::new());
    run_constructor(contract_package_hash);

    // wrap the contract hash so that it can be reached from the test environment
    runtime::put_key("registrar_contract_hash", contract_hash.into());
    runtime::put_key(
        "registrar_contract_hash_wrapped",
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        "registrar_contract_package_hash",
        contract_package_hash.into(),
    );
    runtime::put_key(
        "registrar_contract_package_hash_wrapped",
        storage::new_uref(contract_package_hash).into(),
    );
}

fn run_constructor(contract_package_hash: ContractPackageHash) {
    let partners: Vec<PublicKey> = runtime::get_named_arg("partners");
    let constructor_access: URef = storage::create_contract_user_group(
        contract_package_hash,
        "constructor",
        1,
        Default::default(),
    )
    .unwrap_or_revert_with(RegistrarError::ConstructorGroupCreation)
    .pop()
    .unwrap_or_revert_with(RegistrarError::ConstructorGroupPopEmpty);
    let _: () = runtime::call_versioned_contract(
        contract_package_hash,
        None,
        "constructor",
        runtime_args! {"partners" => partners},
    );
    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(contract_package_hash, "constructor", urefs)
        .unwrap_or_revert_with(RegistrarError::ConstructorGroupRemove);
}

fn get_entry_points() -> EntryPoints {
    let mut eps = EntryPoints::new();
    eps.add_entry_point(EntryPoint::new(
        "constructor",
        vec![Parameter::new(
            "partners",
            CLType::List(Box::new(CLType::PublicKey)),
        )],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        casper_types::EntryPointType::Contract,
    ));
    eps.add_entry_point(EntryPoint::new(
        "register",
        vec![
            Parameter::new("associate_public_key", CLType::Key),
            Parameter::new("associate_type", CLType::U32),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    eps.add_entry_point(EntryPoint::new(
        "unregister",
        vec![Parameter::new("associate_public_key", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    eps.add_entry_point(EntryPoint::new(
        "get_caller_association",
        vec![],
        CLType::U32,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    eps.add_entry_point(EntryPoint::new(
        "get_associate_type",
        vec![Parameter::new("associate_public_key", CLType::Key)],
        CLType::U32,
        EntryPointAccess::Public,
        casper_types::EntryPointType::Contract,
    ));
    eps
}
