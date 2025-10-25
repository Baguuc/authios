pub struct RootUserCheckPasswordParams<'p> {
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String
}
