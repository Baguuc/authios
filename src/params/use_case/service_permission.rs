/// represents parameters needed to create a service permission 
///
pub struct ServicePermissionCreateParams<'p> {
    /// id of the service the permission is for
    ///
    pub service_id: &'p String,
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String
}

/// represents parameters needed to delete a service permission 
///
pub struct ServicePermissionDeleteParams<'p> {
    /// id of the service the permission is for
    ///
    pub service_id: &'p String,
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String
}
