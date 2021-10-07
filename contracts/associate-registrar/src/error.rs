use casper_types::ApiError;

pub enum RegistrarError {
    // errors in associates.rs
    AssociateTypeNonExistant = 0,
    InitAssociatesDictionary = 10,
    InitPublicKeyDictionary = 11,
    OpenAssociatesDictinonaryKey = 20,
    OpenAssociatesDictinonaryURef = 21,
    OpenPublicKeyDictinonaryKey = 22,
    OpenPublicKeyDictinonaryURef = 23,
    // errors is main.rs
    ConstructorGroupCreation = 30,
    ConstructorGroupPopEmpty = 31,
    ConstructorGroupRemove = 32,
}

impl From<RegistrarError> for ApiError {
    fn from(err: RegistrarError) -> ApiError {
        ApiError::User(err as u16)
    }
}
