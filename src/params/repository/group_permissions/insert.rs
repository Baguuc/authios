/// # GroupPermissionInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::group_permissions::GroupPermissionsRepository::insert] method.
///
pub struct GroupPermissionInsertParams {
    /// the name of the permission to revoke
    ///
    pub permission_name: String,
    /// the name of the group to revoke the permission from
    ///
    pub group_name: String,
}
