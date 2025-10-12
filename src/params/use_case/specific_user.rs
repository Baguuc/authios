/// represents params needed to grant user a resource permission
///
pub struct SpecificUserGrantResourcePermissionParams<'p> {
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
pub struct SpecificUserRevokeResourcePermissionParams<'p> {
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
pub struct SpecificUserGetInfoParams<'p> {
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
pub struct SpecificUserDeleteParams<'p> {
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
pub struct SpecificUserUpdateParams<'p> {
    /// user id
    ///
    pub id: &'p i32,
    /// password provided by the user
    ///
    pub password: &'p String,
    /// root password from the config
    ///
    pub root_password: &'p String,
    /// new login (no change when None)
    ///
    pub new_login: &'p Option<String>,
    /// new password (no change when None)
    ///
    pub new_password: &'p Option<String>
}

/// represents params needed to list user's resource permissions as admin
///
pub struct SpecificUserListResourcePermissionsParams<'p> {
    /// id of the user
    ///
    pub id: &'p i32,
    /// password provided by user
    ///
    pub password: &'p String,
    /// root password from the config
    ///
    pub root_password: &'p String,
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

/// represents params needed to check user's permission as admin
///
pub struct SpecificUserCheckResourcePermissionParams<'p> {
    /// id of the user to check permission of
    ///
    pub id: &'p i32,
    /// password provided by the user
    ///
    pub password: &'p String,
    /// root password specified in the config
    ///
    pub root_password: &'p String,
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

/// represents params needed to grant user a service permission
///
pub struct SpecificUserGrantServicePermissionParams<'p> {
    /// id of the user to grant the permission to
    ///
    pub user_id: &'p i32,
    /// id of the service of the permission to grant
    ///
    pub service_id: &'p String,
    /// password provided to the user
    ///
    pub password: &'p String,
    /// root password from config
    ///
    pub root_password: &'p String
}

/// represents params needed to revoke a service permission form user
///
pub struct SpecificUserRevokeServicePermissionParams<'p> {
    /// id of the user to grant the permission to
    ///
    pub user_id: &'p i32,
    /// id of the service of the permission to revoke
    ///
    pub service_id: &'p String,
    /// password provided to the user
    ///
    pub password: &'p String,
    /// root password from config
    ///
    pub root_password: &'p String
}

pub struct SpecificUserCheckServicePermissionParams<'p> {
    /// id of the user to check permission of
    ///
    pub id: &'p i32,
    /// id of the service to filter by
    ///
    pub service_id: &'p String,
    /// password provided by the user
    ///
    pub password: &'p String,
    /// root password specified in the config
    ///
    pub root_password: &'p String
}
