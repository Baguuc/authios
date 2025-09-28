/// params needed to retrieve a resource permission from the database
///
pub struct ResourcePermissionRetrieveParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String,
    /// type of the resource the permission is related to
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String
}

/// params needed to insert a resource permission into the database
///
pub struct ResourcePermissionInsertParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String,
    /// type of the resource the permission is related to
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String
}

/// params needed to delete a resource permission from the database
///
pub struct ResourcePermissionDeleteParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String,
    /// type of the resource the permission is related to
    ///
    pub resource_type: &'p String,
    /// name of the permission
    ///
    pub permission_name: &'p String
}
