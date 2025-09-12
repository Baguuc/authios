/// # UserRevokeGroupParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::revoke_group] method.
///
pub struct UserRevokeGroupParams {
    /// name of the group to revoke
    ///
    pub group_name: String,
    /// login of the user to revoke the group from
    ///
    pub user_login: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
