/// Represents any of the errors that can happen during retrieving user data from JWT token
/// 
#[derive(thiserror::Error, Debug)]
pub enum LoggedUserInfoError {
    /// the token is invalid meaning is in a wrong format or points to a user that doesn't exist
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during deleting a user as himself
/// 
#[derive(thiserror::Error, Debug)]
pub enum LoggedUserDeleteError {
    /// the token is invalid
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during updating user data
/// 
#[derive(thiserror::Error, Debug)]
pub enum LoggedUserUpdateError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during listing user's resource permissions
/// 
#[derive(thiserror::Error, Debug)]
pub enum LoggedUserListResourcePermissionsError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during checking if user is permitted to operation
/// on resource permissions
/// 
#[derive(thiserror::Error, Debug)]
pub enum LoggedUserCheckResourcePermissionError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// the permission to check for is not found
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound
}
