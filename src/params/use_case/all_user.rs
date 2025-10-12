/// represents params needed to log in as a user
///
pub struct AllUserLoginParams<'p> {
    /// user login
    ///
    pub login: &'p String,
    /// user password
    ///
    pub password: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String
}

/// represents params needed to register a user
///
pub struct AllUserRegisterParams<'p> {
    /// user login
    ///
    pub login: &'p String,
    /// user password
    ///
    pub password: &'p String
}
