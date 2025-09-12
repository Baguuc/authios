/// # GroupCreateParams
///
/// Represent parameters for using the
/// [crate::use_cases::group::GroupsUseCase::create] method.
///
pub struct GroupCreateParams {
    /// name of the group to create
    ///
    pub name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
