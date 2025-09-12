/// # authios::errors::use_case::user::authorize::UserAuthorizeError
///
/// represents one of errors that can occur while trying to check if user has a permission using
/// [crate::use_cases::user::UsersUseCase::authorize] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserAuthorizeError {
    /// # UserAuthorizeError::InvalidToken
    ///
    /// This means that the token provided by the user is invalid, potentially in a wrong format,
    /// structure or containing info about user that doesn't exist.
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// # UserAuthorizeError::PermissionNotFound
    ///
    /// This means that the permission to check for do not exist in the database.
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// # UserAuthorizeError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
