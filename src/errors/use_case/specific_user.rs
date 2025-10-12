/// Represents any of the errors that can happen during granting user a resource permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserGrantResourcePermissionError {
    /// the permission to grant is not found
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
pub enum SpecificUserRevokeResourcePermissionError {
    /// the permission to revoke is not found
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

/// Represents any of the errors that can happen during getting user info as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserGetInfoError {
    /// the user with provided id not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during deleting a user as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserDeleteError {
    /// the user with provided id not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during updating user data as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserUpdateError {
    /// the user with provided id not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during listing user's resource permissions as
/// admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserListResourcePermissionsError {
    /// the user with specified id is not found
    ///
    #[error("NOT_FOUND")]
    NotFound,
    /// the user sending request is not authorized to do this operation
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during checking user's resource permission as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserCheckResourcePermissionError {
    /// the user with specified id is not found
    ///
    #[error("NOT_FOUND")]
    NotFound,
    /// the permission to check for is not found
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// the user sending request is not authorized to do this operation
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during granting user a service permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserGrantServicePermissionError {
    /// the root password is invalid
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission to grant is not found
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// user with specified id is not found in the database
    ///
    #[error("NOT_ADDED_YET")]
    UserNotFound,
    /// user already has that permission
    ///
    #[error("ALREADY_ADDED")]
    AlreadyAdded
}

/// Represents any of the errors that can happen during granting user a service permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserRevokeServicePermissionError {
    /// the root password is invalid
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// the permission to revoke is not found
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// user with specified id is not found in the database
    ///
    #[error("NOT_ADDED_YET")]
    UserNotFound,
    /// user already has that permission
    ///
    #[error("NOT_ADDED_YET")]
    NotAddedYet
}

/// Represents any of the errors that can happen during checking user's service permission as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum SpecificUserCheckServicePermissionError {
    /// the user with specified id is not found
    ///
    #[error("NOT_FOUND")]
    NotFound,
    /// the permission to check for is not found
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    /// the user sending request is not authorized to do this operation
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}
