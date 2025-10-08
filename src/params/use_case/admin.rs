/// represents params needed to grant user a resource permission
///
pub struct AdminGrantUserResourcePermissionParams<'p> {
    /// id of the user to grant the permission to
    ///
    pub user_id: &'p i32,
    /// id of the service of the resource permission to grant
    ///
    pub service_id: &'p String,
    /// type of the resource of the resource permission to grant
    ///
    pub resource_type: &'p String,
    /// name of the permission of the resource permission to grant
    ///
    pub permission_name: &'p String,
    /// id of the resource to grant the permission for
    ///
    pub resource_id: &'p i32,
    /// password provided to the user
    ///
    pub password: &'p String,
    /// root password from config
    ///
    pub root_password: &'p String
}

/// represents params needed to grant user a resource permission
///
pub struct AdminRevokeUserResourcePermissionParams<'p> {
    /// id of the user to revoke the permission to
    ///
    pub user_id: &'p i32,
    /// id of the service of the resource permission to revoke
    ///
    pub service_id: &'p String,
    /// type of the resource of the resource permission to revoke
    ///
    pub resource_type: &'p String,
    /// name of the permission of the resource permission to revoke
    ///
    pub permission_name: &'p String,
    /// id of the resource to revoke the permission from
    ///
    pub resource_id: &'p i32,
    /// password provided to the user
    ///
    pub password: &'p String,
    /// root password from config
    ///
    pub root_password: &'p String
}

/// represents params needed to delete a user as admin
///
pub struct AdminGetUserInfoParams<'p> {
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

/// represents params needed to delete a user as admin
///
pub struct AdminDeleteUserParams<'p> {
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
