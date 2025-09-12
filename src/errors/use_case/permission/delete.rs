/// # authios::errors::use_case::permission::delete::PermissionDeleteError
/// 
/// Represents one of errors that can occur during deletion of a permission using
/// [crate::use_cases::permission::PermissionsUseCase::delete] method.
///
#[derive(thiserror::Error, Debug)]
pub enum PermissionDeleteError {
    /// ## PermissionDeleteError::NotFound
    ///
    /// This means that the permission user was trying to delete do not exist.
    ///
    #[error("NOT_FOUND")]
    NotFound,
    /// ## PermissionDeleteError::Unauthorized
    ///
    /// This means that user trying to delete a permission do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## PermissionDeleteError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
