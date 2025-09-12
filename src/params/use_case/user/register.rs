/// # UserRegisterParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::register] method.
///
pub struct UserRegisterParams {
    /// login of the user to register
    ///
    pub login: String,
    /// password of the user to register
    ///
    pub pwd: String
}
