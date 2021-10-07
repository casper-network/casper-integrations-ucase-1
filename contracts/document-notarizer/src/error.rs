use casper_types::ApiError;

pub enum NotarizerError {
    // Errors in associate_type.rs
    AssociateTypeNonExistant = 100,
    InitDocumentsDictionaryURef = 110,
    InitSubmitterDictionaryURef = 111,
    OpenDocumentsDictionaryKey = 120,
    OpenSubmitterDictionaryKey = 121,
    OpenDocumentsDictionaryURef = 122,
    OpenSubmitterDictionaryURef = 123,
    NotarizeDocumentKeyNotHash = 130,
    GetDocumentMetaKeyNotHash = 140,
    // Errors is main.rs
    NotarizeDocumentNoAccess = 150,
    GetDocumentMetaNoAccessRights = 160,
    GetRegistrarKeyNotFound = 170,
    GetRegistrarReadError = 171,
    GetRegistrarReadNotFound = 172,
    GetRegistrarKeyNotURef = 173,
    ConstructorGroupCreation = 180,
    ConstructorGroupPopEmpty = 181,
    ConstructorGroupRemove = 182,
}

impl From<NotarizerError> for ApiError {
    fn from(err: NotarizerError) -> ApiError {
        ApiError::User(err as u16)
    }
}
