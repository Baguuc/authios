/// # UserUpdatePwdParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::update_pwd] method.
///
pub struct UserUpdatePwdParams {
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String,
    /// new password of the user
    ///
    pub new_pwd: String
}
