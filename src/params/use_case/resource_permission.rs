pub struct ResourcePermissionCreateParams<'p> {
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String,
    /// id of the service of the permission
    ///
    pub service_id: &'p String,
    /// type of the resource of the permission
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
    /// id of the service of the permission
    ///
    pub service_id: &'p String,
    /// type of the resource of the permission
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String,
}

pub struct ResourcePermissionListUsersParams<'p> {
    /// the password provided by user
    ///
    pub password: &'p String,
    /// the root password from config
    ///
    pub root_password: &'p String,
    /// id of the service of the permission
    ///
    pub service_id: &'p String,
    /// type of the resource of the permission
    ///
    pub resource_type: &'p String,
    /// type of the resource of the permission
    ///
    pub resource_id: &'p String,
    /// number of the page for pagination.
    /// the page size is 5.
    ///
    pub page_number: &'p u32
}
