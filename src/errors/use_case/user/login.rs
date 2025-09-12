/// # authios::errors::use_case::user::login::UserLoginError
///
/// represents one of errors that can occur while trying to log in as some user using
/// [crate::use_cases::user::UsersUseCase::login] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserLoginError {
    /// ## UserLoginError::UserNotFound
    ///
    /// This means that provided user do not exist.
    ///
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    /// ## UserLoginError::WrongPassword
    ///
    /// This means that provided password do not match the one in database that is associated with provided user.
    ///
    #[error("WRONG_PASSWORD")]
    WrongPassword,
    /// ## UserLoginError::CannotGenerateToken
    ///
    /// This means that server cannot generate a session token for some reason.
    ///
    #[error("CANNOT_GENERATE_TOKEN")]
    CannotGenerateToken,
    /// ## UserLoginError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
