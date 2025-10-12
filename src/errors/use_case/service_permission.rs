/// represents one of the errors that can occur while trying to create a service permission
///
#[derive(thiserror::Error, Debug)]
pub enum ServicePermissionCreateError {
    /// provided password do not match the one from the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission for specified service already exists
    ///
    #[error("ALREADY_EXISTS")]
    AlreadyExists
}

/// represents one of the errors that can occur while trying to delete a service permission
///
#[derive(thiserror::Error, Debug)]
pub enum ServicePermissionDeleteError {
    /// provided password do not match the one from the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission for specified service is not found in the database
    ///
    #[error("NOT_FOUND")]
    NotFound
}
