/// # GroupGrantPermissionParams
///
/// Represent parameters for using the
/// [crate::use_cases::group::GroupsUseCase::grant_permission] method.
///
pub struct GroupGrantPermissionParams {
    /// name of the permission to grant
    ///
    pub permission_name: String,
    /// name of the group to grant the permission to
    ///
    pub group_name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
