/// Represents any of the errors that can happen during retrieving user data from JWT token
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserRetrieveFromTokenError {
    /// the token is invalid meaning is in a wrong format or points to a user that doesn't exist
    ///
    #[error("INVALID_TOKEN")] InvalidToken }

/// Represents any of the errors that can happen during logging in
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserLoginError {
    /// the user with provided login not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in database
    ///
    #[error("WRONG_PASSWORD")]
    WrongPassword
}

/// Represents any of the errors that can happen during registering a user
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserRegisterError {
    /// the user with provided login already exist
    ///
    #[error("ALREADY_EXIST")] 
    AlreadyExist
}

/// Represents any of the errors that can happen during deleting a user as admin
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserDeleteAsAdminError {
    /// the user with provided login not exist
    ///
    #[error("NOT_FOUND")] 
    NotFound,
    /// the password do not match the one in the config
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized
}

/// Represents any of the errors that can happen during deleting a user as himself
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserDeleteAsSelfError {
    /// the token is invalid
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during updating user data
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserUpdateError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during listing user's resource permissions
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserListResourcePermissionsError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken
}

/// Represents any of the errors that can happen during checking if user is permitted to operation
/// on resource permissions
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserCheckResourcePermissionError {
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("INVALID_TOKEN")]
    InvalidToken,
    /// the token is invalid, meaning is in a wrong format or pointing to null user
    ///
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound
}

/// Represents any of the errors that can happen during granting user a resource permission
/// 
#[derive(thiserror::Error, Debug)]
pub enum UserGrantResourcePermissionError {
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
pub enum UserRevokeResourcePermissionError {
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
