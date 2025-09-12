/// # UserGroupDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::user_groups::UserGroupsRepository::delete] method.
///
pub struct UserGroupDeleteParams {
    /// the name of the group to revoke
    ///
    pub group_name: String,
    /// the name of the user to revoke the group from
    ///
    pub user_login: String,
}
