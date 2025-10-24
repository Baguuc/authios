#[derive(thiserror::Error, Debug)]
pub enum ResourcePermissionCreateError {
    /// provided password do not match the one from the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission with specified name, service id and resource type already exists
    ///
    #[error("ALREADY_EXISTS")]
    AlreadyExists
}

#[derive(thiserror::Error, Debug)]
pub enum ResourcePermissionDeleteError {
    /// provided password do not match the one from the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission with specified name, service id and resource type is not found in the
    /// database
    ///
    #[error("NOT_FOUND")]
    NotFound
}

#[derive(thiserror::Error, Debug)]
pub enum ResourcePermissionListUsersError {
    /// provided password do not match the one from the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission with specified name, service id and resource type is not found in the
    /// database
    ///
    #[error("NOT_FOUND")]
    PermissionNotFound
}
