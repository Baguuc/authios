/// params needed to list user's resource permissions from the database
///
pub struct UserResourcePermissionListParams<'p> {
    /// id of the user to filter by
    ///
    pub user_id: &'p i32,
    /// id of the service to filter by
    ///
    pub service_id: &'p String,
    /// type of the resource to filter by
    ///
    pub resource_type: &'p String,
    /// number of the page for pagination.
    /// the page size is 5.
    ///
    pub page_number: &'p Option<u32>,
}

/// params needed to get page count for pagination from list method
///
pub struct UserResourcePermissionGetPageCountParams<'p> {
    /// id of the user to filter by
    ///
    pub user_id: &'p i32,
    /// id of the service to filter by
    ///
    pub service_id: &'p String,
    /// type of the resource to filter by
    ///
    pub resource_type: &'p String
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
    pub resource_id: &'p String,
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
    pub resource_id: &'p String,
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
    pub resource_id: &'p String,
}
