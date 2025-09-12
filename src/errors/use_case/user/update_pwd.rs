/// # authios::errors::use_case::user::revoke_group::UserUpdatePwdError
///
/// represents one of errors that can occur while trying to update user's password using
/// [crate::use_cases::user::UsersUseCase::update_pwd] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserUpdatePwdError {
    /// # UserUpdatePwdError::InvalidToken
    ///
    /// This means that the token provided by the user is invalid, potentially in a wrong format,
    /// structure or containing info about user that doesn't exist.
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// ## UserUpdatePwdError::CannotHashPassword
    ///
    /// This means that the password provided for this user cannot be hashed.
    ///
    #[error("CANNOT_HASH")]
    CannotHash,
    /// ## UserUpdatePwdError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
