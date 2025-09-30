pub struct ResourcePermissionCreateParams<'p> {
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String,
    /// id of the service of the resource permission
    ///
    pub service_id: &'p String,
    /// type of the resource of the service permission
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String,
}

pub struct ResourcePermissionDeleteParams<'p> {
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String,
    /// id of the service of the resource permission
    ///
    pub service_id: &'p String,
    /// type of the resource of the service permission
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String,
}
