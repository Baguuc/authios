/// # UserAuthorizeParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::authorize] method.
///
pub struct UserAuthorizeParams {
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String,
    /// name of the permission to check for
    ///
    pub permission_name: String
}
