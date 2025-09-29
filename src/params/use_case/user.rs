/// represents params needed to retrieve user's data from JWT session token
///
pub struct UserRetrieveFromTokenParams<'p> {
    /// JWT token provided by the user
    ///
    pub token: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String
}

/// represents params needed to log in as a user
///
pub struct UserLoginParams<'p> {
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
pub struct UserRegisterParams<'p> {
    /// user login
    ///
    pub login: &'p String,
    /// user password
    ///
    pub password: &'p String
}

/// represents params needed to delete a user as admin
///
pub struct UserDeleteAsAdminParams<'p> {
    /// user id
    ///
    pub id: &'p i32,
    /// password provided by the user
    ///
    pub password: &'p String,
    /// root password from the config
    ///
    pub root_password: &'p String
}

/// represents params needed to delete a user as himself
///
pub struct UserDeleteAsSelfParams<'p> {
    /// JWT session token
    ///
    pub token: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String
}

/// represents params needed to update user's data
///
pub struct UserUpdateParams<'p> {
    /// JWT session token
    ///
    pub token: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String,
    /// new login (no change when None)
    ///
    pub new_login: Option<String>,
    /// new password (no change when None)
    ///
    pub new_password: Option<String>,
}
