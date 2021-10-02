use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    PublicKey, RuntimeArgs, SecretKey, U512,
};

use crate::{OPERATOR, USER};

pub struct RegistrarAndNotarizer {
    pub context: TestContext,
    pub registrar_hash: Hash,
    pub notarizer_hash: Hash,
    pub notarizer_package_hash: ContractPackageHash,
    pub operator: AccountHash,
    pub operator_key: PublicKey,
    pub user: AccountHash,
    pub user_key: PublicKey,
}

impl RegistrarAndNotarizer {
    pub fn deploy(mut partners: Vec<PublicKey>) -> Self {
        // Deploy Registrar
        let operator_key: PublicKey = (&SecretKey::ed25519_from_bytes(OPERATOR).unwrap()).into();
        let operator = AccountHash::from(&operator_key);
        let user_key: PublicKey = (&SecretKey::ed25519_from_bytes(USER).unwrap()).into();
        let user = AccountHash::from(&user_key);
        let mut context = TestContextBuilder::new()
            .with_public_key(operator_key.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(user_key.clone(), U512::from(500_000_000_000_000_000u64))
            .build();
        partners.push(operator_key.clone());
        let session_code = Code::from("associate-registrar.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {"partners"=>partners})
            .with_address(operator)
            .with_authorization_keys(&[operator])
            .build();
        context.run(session);
        let registrar_hash = context
            .query(operator, &["registrar_contract_hash_wrapped".into()])
            .unwrap()
            .into_t()
            .unwrap();
        let registrar_package: ContractPackageHash = context
            .query(
                operator,
                &["registrar_contract_package_hash_wrapped".into()],
            )
            .unwrap()
            .into_t()
            .unwrap();
        // Deploy State Manager
        let session_code = Code::from("document-notarizer.wasm");
        let session = SessionBuilder::new(
            session_code,
            runtime_args! {"registrar" => Key::Hash(registrar_package.value())},
        )
        .with_address(operator)
        .with_authorization_keys(&[operator])
        .build();
        context.run(session);
        let notarizer_hash = context
            .query(
                operator,
                &["document_notarizer_contract_hash_wrapped".into()],
            )
            .unwrap()
            .into_t()
            .unwrap();
        let notarizer_package_hash = context
            .query(
                operator,
                &["document_notarizer_contract_package_hash_wrapped".into()],
            )
            .unwrap()
            .into_t()
            .unwrap();

        Self {
            context,
            registrar_hash,
            notarizer_hash,
            notarizer_package_hash,
            operator,
            operator_key,
            user,
            user_key,
        }
    }

    fn call(&mut self, caller: AccountHash, code: Code, args: RuntimeArgs) {
        let session = SessionBuilder::new(code, args)
            .with_address(caller)
            .with_authorization_keys(&[caller])
            .build();
        self.context.run(session);
    }

    fn call_registrar(&mut self, caller: AccountHash, method: &str, args: RuntimeArgs) {
        let code = Code::Hash(self.registrar_hash, method.to_string());
        self.call(caller, code, args);
    }

    fn call_notarizer(&mut self, caller: AccountHash, method: &str, args: RuntimeArgs) {
        let code = Code::Hash(self.notarizer_hash, method.to_string());
        self.call(caller, code, args);
    }

    pub fn test_get_document_meta(
        &mut self,
        caller: AccountHash,
        document_hash: Key,
        expected_result: Option<Vec<String>>,
    ) {
        let session_code = Code::from("get_document_meta.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {"document_hash" => document_hash, "expected_result"=> expected_result, "contract_hash" => self.notarizer_package_hash})
            .with_address(caller)
            .with_authorization_keys(&[caller])
            .build();
        self.context.run(session);
    }

    pub fn notarize_document(
        &mut self,
        caller: AccountHash,
        document_hash: Key,
        document_meta: Option<Vec<String>>,
    ) {
        self.call_notarizer(
            caller,
            "notarize_document",
            runtime_args! {"document_hash" => document_hash, "document_meta" => document_meta},
        )
    }

    pub fn registrar_register(
        &mut self,
        caller: AccountHash,
        associate_public_key: PublicKey,
        associate_type: u32,
    ) {
        self.call_registrar(
            caller,
            "register",
            runtime_args! {"associate_public_key" => associate_public_key, "associate_type" => associate_type },
        )
    }

    pub fn registrar_unregister(&mut self, caller: AccountHash, associate_public_key: PublicKey) {
        self.call_registrar(
            caller,
            "unregister",
            runtime_args! {"associate_public_key" => associate_public_key},
        )
    }

    pub fn query_dictionary_value<T: CLTyped + FromBytes>(
        &self,
        dict_name: &str,
        key: &str,
    ) -> Option<T> {
        match self.context.query_dictionary_item(
            Key::Hash(self.registrar_hash),
            Some(dict_name.to_string()),
            key.to_string(),
        ) {
            Err(_) => None,
            Ok(maybe_value) => maybe_value.into_t().unwrap(),
        }
    }
}
