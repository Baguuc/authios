/// params needed to list user's resource permissions from the database
///
pub struct UserResourcePermissionListParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the service
    ///
    pub service_id: &'p String,
    /// type of the resource
    ///
    pub resource_type: &'p String,
}

/// params needed to retrieve user's resource permission from the database
///
pub struct UserResourcePermissionRetrieveParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the resource permission
    ///
    pub permission_id: &'p i32,
    /// id of the resource
    ///
    pub resource_id: &'p i32,
}

/// params needed to grant a user a resource permission
///
pub struct UserResourcePermissionInsertParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the resource permission
    ///
    pub resource_permission_id: &'p i32,
    /// id of the resource
    ///
    pub resource_id: &'p i32,
}

/// params needed to revoke a user a resource permission
///
pub struct UserResourcePermissionDeleteParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the resource permission
    ///
    pub resource_permission_id: &'p i32,
    /// id of the resource
    ///
    pub resource_id: &'p i32,
}
