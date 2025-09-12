/// # authios::errors::use_case::groups::revoke_permission::GroupRevokePermissionError
///
/// Represents one of errors that can happen while revoking a permission from group using
/// [crate::use_cases::group::GroupsUseCase::revoke_permission] method.
///
#[derive(thiserror::Error, Debug)]
pub enum GroupRevokePermissionError {
    /// ## GroupRevokePermissionError::PermissionNotFound
    ///
    /// This means that the permission to be revoked do not exist.
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// ## GroupRevokePermissionError::PermissionNotFound
    ///
    /// This means that the group to revoke the permission from do not exist.
    ///
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    /// ## GroupRevokePermissionError::PermissionNotFound
    ///
    /// This means that the permission is not yet added to the provided group.
    ///
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    /// ## GroupRevokePermissionError::Unauthorized
    ///
    /// This means that user trying to revoke a permission do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## GroupRevokePermissionError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
