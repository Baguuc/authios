/// # UserInfoParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::info] method.
///
pub struct UserInfoParams {
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
