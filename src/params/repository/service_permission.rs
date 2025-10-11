/// params needed to retrieve a service permission from the database
///
pub struct ServicePermissionRetrieveParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String,
}

/// params needed to insert a service permission into the database
///
pub struct ServicePermissionInsertParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String
}

/// params needed to delete a service permission from the database
///
pub struct ServicePermissionDeleteParams<'p> {
    /// id of the service the permission is related to
    ///
    pub service_id: &'p String
}
