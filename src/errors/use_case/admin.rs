/// Represents any of the errors that can happen during granting user a resource permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum AdminGrantUserResourcePermissionError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// the root password is invalid
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// user with specified id is not found in the database
    ///
    #[error("NOT_ADDED_YET")]
    UserNotFound,
    /// user already has that permission for specified resource
    ///
    #[error("ALREADY_ADDED")]
    AlreadyAdded
}

/// Represents any of the errors that can happen during revoking user a resource permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum AdminRevokeUserResourcePermissionError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// the root password is invalid
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// user with specified id is not found in the database
    ///
    #[error("NOT_ADDED_YET")]
    UserNotFound,
    /// user already has that permission for specified resource
    ///
    #[error("NOT_ADDED_YET")]
    NotAddedYet
}

/// Represents any of the errors that can happen during deleting a user as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum AdminDeleteUserError {
    /// the user with provided login not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}
