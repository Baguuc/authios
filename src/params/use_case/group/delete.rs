/// # GroupDeleteParams
///
/// Represent parameters for using the
/// [crate::use_cases::group::GroupsUseCase::delete] method.
///
pub struct GroupDeleteParams {
    /// name of the group to delete
    ///
    pub name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}
