/// # authios::errors::use_case::group::grant_permission::GroupGrantPermissionError
///
/// Represents one of errors that can happen while granting a permission to group using
/// [crate::use_cases::group::GroupsUseCase::grant_permission] method.
///
#[derive(thiserror::Error, Debug)]
pub enum GroupGrantPermissionError {
    /// ## GroupGrantPermissionError::PermissionNotFound
    ///
    /// This means that the permission to be granted do not exist.
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// ## GroupGrantPermissionError::GroupNotFound
    ///
    /// This means that the group to be granted do not exist.
    ///
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    /// ## GroupGrantPermissionError::AlreadyAdded
    ///
    /// This means that the permission is already granted to that group.
    ///
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    /// ## GroupGrantPermissionError::Unauthorized
    ///
    /// This means that user trying to grant the permission do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## GroupGrantPermissionError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
