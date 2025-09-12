/// # PermissionCreateParams
///
/// Represent parameters for using the
/// [crate::use_cases::permission::PermissionsUseCase::create] method.
///
pub struct PermissionCreateParams {
    /// name of the permission to create
    ///
    pub name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
