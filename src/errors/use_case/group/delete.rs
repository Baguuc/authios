/// # GroupDeleteError
/// 
/// Represents one of errors that can happen while deleting a group using
/// [crate::use_cases::group::GroupsUseCase::delete] method.
///
#[derive(thiserror::Error, Debug)]
pub enum GroupDeleteError {
    /// ## GroupDeleteError::NotFound
    ///
    /// This means that the group user was trying to delete do not exist.
    ///
    #[error("NOT_FOUND")]
    NotFound,
    /// ## GroupDeleteError::Unauthorized
    ///
    /// This means that user trying to delete a group do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## GroupDeleteError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
