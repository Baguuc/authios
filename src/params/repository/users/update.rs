/// # UserUpdateParams
///
/// Represent parameters for using the
/// [crate::repositories::users::UsersRepository::update] method.
///
pub struct UserUpdateParams {
    /// login of the user to update the data of
    ///
    pub user_login: String,
    /// new password hash to replace existing with
    ///
    pub new_pwd: String
}
