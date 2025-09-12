/// # authios::errors::use_case::permission::create::PermissionCreateError
///
/// Represents one of errors that can occur while creating a permission using the
/// [crate::use_cases::permission::PermissionsUseCase::create] method.
///
#[derive(thiserror::Error, Debug)]
pub enum PermissionCreateError {
    /// ## PermissionCreateError::AlreadyExist
    ///
    /// This means that a permission with provided name is already found in the database - thus cannot
    /// be created as it will violate the UNIQUE constraint of permission names.
    ///
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    /// ## PermissionCreateError::RootGroupNotFound
    ///
    /// This means that the root group, created after initing the system with `authios init`
    /// command is not found, and so the permission cannot be granted to it (it is required that
    /// all of the permissions will be granted to this group).
    ///
    #[error("ROOT_GROUP_NOT_FOUND")]
    RootGroupNotFound,
    /// ## PermissionCreateError::Unauthorized
    ///
    /// This means that user trying to create a permission do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## PermissionCreateError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
