/// # authios::errors::use_case::user::grant_group::UserGrantGroupError
///
/// represents one of errors that can occur while trying to grant user a permission using
/// [crate::use_cases::user::UsersUseCase::grant_group] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserGrantGroupError {
    /// ## UserGrantGroupError::GroupNotFound
    ///
    /// This means that the group to be granted do not exist in the database.
    ///
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    /// ## UserGrantGroupError::UserNotFound
    ///
    /// This means that the provided user do not exist in the database.
    ///
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    /// ## UserGrantGroupError::AlreadyAdded
    ///
    /// This means that the user already has provided group granted.
    ///
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    /// ## UserGrantGroupError::Unauthorized
    ///
    /// This means that user trying to grant someone a group do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## UserGrantGroupError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
