/// # PermissionDeleteParams
///
/// Represent parameters for using the
/// [crate::use_cases::permission::PermissionsUseCase::delete] method.
///
pub struct PermissionDeleteParams {
    /// name of the permission to delete
    ///
    pub name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
