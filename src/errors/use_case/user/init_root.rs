/// # authios::errors::use_case::user::init_root::UserInitRootError
///
/// represents one of errors that can occur while trying to init root user and related data using
/// [crate::use_cases::user::UsersUseCase::init_root] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserInitRootError {
    /// ## UserInitRootError::CannotHashPassword
    ///
    /// This means that the password provided for root user cannot be hashed.
    ///
    #[error("Cannot hash root user's password from config")]
    CannotHashPassword,
    /// ## UserInitRootError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("Database connection cannot be acquired.")]
    DatabaseConnection
}
