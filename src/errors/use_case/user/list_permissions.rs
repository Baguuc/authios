/// # authios::errors::use_case::user::list_permissions::UserListPermissionsError
///
/// represents one of errors that can occur while trying to list user's permissions using
/// [crate::use_cases::user::UsersUseCase::list_permissions] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserListPermissionsError {
    /// ## UserListPermissionsError::InvalidToken
    ///
    /// This means that the token provided by the user is invalid, potentially in a wrong format,
    /// structure or containing info about user that doesn't exist.
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// # UserListPermissionsError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
