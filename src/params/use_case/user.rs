/// represents params needed to retrieve user's data from JWT session token
///
pub struct UserInfoParams<'p> {
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
    pub new_login: &'p Option<String>,
    /// new password (no change when None)
    ///
    pub new_password: &'p Option<String>
}

/// represents params needed to list user's resource permissions
///
pub struct UserListResourcePermissionsParams<'p> {
    /// JWT session token
    ///
    pub token: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String,
    /// id of the service to filter by
    ///
    pub service_id: &'p String,
    /// type of the resource to filter by
    ///
    pub resource_type: &'p String,
    /// number of the page for pagination.
    /// the page size is 5.
    ///
    pub page_number: &'p u32,
}

/// represents params needed to check if user is permitted to operation on resource
///
pub struct UserCheckResourcePermissionParams<'p> {
    /// JWT session token
    ///
    pub token: &'p String,
    /// JWT encryption key set in the config
    ///
    pub jwt_encryption_key: &'p String,
    /// id of the service to filter by
    ///
    pub service_id: &'p String,
    /// type of the resource to filter by
    ///
    pub resource_type: &'p String,
    /// id of the resource to filter by
    ///
    pub resource_id: &'p i32,
    /// name of the permission to check for
    ///
    pub permission_name: &'p String
}

/// represents params needed to grant user a resource permission
///
pub struct UserGrantResourcePermissionParams<'p> {
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
pub struct UserRevokeResourcePermissionParams<'p> {
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
