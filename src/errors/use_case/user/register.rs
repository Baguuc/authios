/// # authios::errors::use_case::user::register::UserRegisterError
///
/// represents one of errors that can occur while trying to register a user using
/// [crate::use_cases::user::UsersUseCase::register] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserRegisterError {
    /// ## UserRegisterError::AlreadyExist
    ///
    /// This means that a user with provided login already exists in the database, so cannot be
    /// created as it would violate the UNIQUE constraint of user logins.
    ///
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    /// ## UserRegisterError::CannotHashPassword
    ///
    /// This means that the password provided for this user cannot be hashed.
    ///
    #[error("CANNOT_HASH_PASSWORD")]
    CannotHashPassword,
    /// ## UserRegisterError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
