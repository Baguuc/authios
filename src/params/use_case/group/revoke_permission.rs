/// # GroupRevokePermissionParams
///
/// Represent parameters for using the
/// [crate::use_cases::group::GroupsUseCase::revoke_permission] method.
///
pub struct GroupRevokePermissionParams {
    /// name of the group to grant the permission to
    ///
    pub permission_name: String,
    /// name of the group to revoke the permission from
    ///
    pub group_name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
