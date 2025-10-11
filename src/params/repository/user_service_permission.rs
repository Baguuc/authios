/// params needed to retrieve user's service permission from the database
///
pub struct UserServicePermissionRetrieveParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the service permission
    ///
    pub permission_id: &'p i32,
}

/// params needed to grant a user a resource permission
///
pub struct UserServicePermissionInsertParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the service permission
    ///
    pub permission_id: &'p i32,
}

/// params needed to revoke an user's service permission
///
pub struct UserServicePermissionDeleteParams<'p> {
    /// id of the user
    ///
    pub user_id: &'p i32,
    /// id of the service permission
    ///
    pub permission_id: &'p i32,
}
