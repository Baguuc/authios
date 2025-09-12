/// # UserLoginParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::login] method.
///
pub struct UserLoginParams {
    /// login of the user to log in as
    ///
    pub login: String,
    /// password of the user to login
    ///
    pub pwd: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
