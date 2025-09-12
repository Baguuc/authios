/// # UserInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::users::UsersRepository::insert] method.
///
pub struct UserInsertParams {
    /// login of the user to insert
    ///
    pub login: String,
    /// hashed password of the user to insert
    ///
    pub pwd: String
}
