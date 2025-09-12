/// # UserGroupInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::user_groups::UserGroupsRepository::insert] method.
///
pub struct UserGroupInsertParams {
    /// the name of the group to grant
    ///
    pub group_name: String,
    /// the name of the user to grant the group to
    ///
    pub user_login: String,
}
