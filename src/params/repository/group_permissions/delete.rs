/// # GroupPermissionDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::group_permissions::GroupPermissionsRepository::delete] method.
///
pub struct GroupPermissionDeleteParams {
    /// the name of the permission to grant
    ///
    pub permission_name: String,
    /// the name of the group to grant the permission to
    ///
    pub group_name: String,
}
