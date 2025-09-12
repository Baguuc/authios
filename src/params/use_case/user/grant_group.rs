/// # UserGrantGroupParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::grant_group] method.
///
pub struct UserGrantGroupParams {
    /// name of the group to grant
    ///
    pub group_name: String,
    /// login of the user to grant the group to
    ///
    pub user_login: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
