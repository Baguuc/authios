/// # authios::errors::use_case::user::info::UserInfoError
///
/// represents one of errors that can occur while trying to get user info using
/// [crate::use_cases::user::UsersUseCase::info] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserInfoError {
    /// ## UserInfoError::InvalidToken
    ///
    /// This means that the token provided by the user is invalid, potentially in a wrong format,
    /// structure or containing info about user that doesn't exist.
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// # UserInfoError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
