/// # GroupCreateError
///
/// Represents one of errors that can occur while creating a group using the
/// [crate::use_cases::group::GroupsUseCase::create] method.
///
#[derive(thiserror::Error, Debug)]
pub enum GroupCreateError {
    /// ## GroupCreateError::AlreadyExist
    ///
    /// This means that a group with provided name is already found in the database - thus cannot
    /// be created as it will violate the UNIQUE constraint of group names.
    ///
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    /// ## GroupCreateError::Unauthorized
    ///
    /// This means that user trying to create a group do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## GroupCreateError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
