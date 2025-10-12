/// Represents any of the errors that can happen during logging in
/// 
#[derive(thiserror::Error, Debug)]
pub enum AllUserLoginError {
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
pub enum AllUserRegisterError {
    /// the user with provided login already exist
    ///
    #[error("ALREADY_EXISTS")] 
    AlreadyExists
}
