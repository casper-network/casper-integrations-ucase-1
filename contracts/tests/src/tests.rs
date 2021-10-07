use casper_types::Key;

use crate::contract_system::RegistrarAndNotarizer;

#[test]
fn test_deploys() {
    RegistrarAndNotarizer::deploy(Vec::new());
}

#[test]
fn test_deployer_partner_user() {
    let mut contracts = RegistrarAndNotarizer::deploy(Vec::new());
    contracts.notarize_document(
        contracts.operator,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
    contracts.test_get_document_meta(
        contracts.operator,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
}

#[test]
fn test_registered_user() {
    let mut contracts = RegistrarAndNotarizer::deploy(Vec::new());
    contracts.registrar_register(contracts.operator, contracts.user_key.clone(), 2);
    contracts.notarize_document(
        contracts.user,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
    contracts.test_get_document_meta(
        contracts.user,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
}

#[test]
#[should_panic = "ApiError::User(150)"]
fn test_notregistered_user() {
    let mut contracts = RegistrarAndNotarizer::deploy(Vec::new());
    // Only associates that have been registered gain access
    contracts.notarize_document(
        contracts.user,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
}

#[test]
#[should_panic = "ApiError::User(150)"]
fn test_unregistered_user() {
    let mut contracts = RegistrarAndNotarizer::deploy(Vec::new());
    contracts.registrar_register(contracts.operator, contracts.user_key.clone(), 2);
    contracts.registrar_unregister(contracts.operator, contracts.user_key.clone());
    // Users whose registration have been removed cannot make changes
    contracts.notarize_document(
        contracts.user,
        Key::Hash([1u8; 32]),
        Some(vec![
            "full_1_hash".to_string(),
            "partner_deployer".to_string(),
            "test".to_string(),
        ]),
    );
}
