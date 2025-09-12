/// # authios::errors::use_case::user::revoke_group::UserRevokeGroupError
///
/// represents one of errors that can occur while trying to register a user using
/// [crate::use_cases::user::UsersUseCase::register] method.
///
#[derive(thiserror::Error, Debug)]
pub enum UserRevokeGroupError {
    /// ## UserRevokeGroupError::GroupNotFound
    ///
    /// This means that the group to be revoked do not exist in the database.
    ///
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    /// ## UserRevokeGroupError::UserNotFound
    ///
    /// This means that the provided user do not exist in the database.
    ///
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    /// ## UserRevokeGroupError::NotAddedYet
    ///
    /// This means that the user do not have provided group granted - thus it cannot be revoked.
    ///
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    /// ## UserRevokeGroupError::Unauthorized
    ///
    /// This means that user trying to revoke a group from someone do not have the "authios:all" permission so
    /// is not authorized to do this operation.
    ///
    #[error("UNAUTHORIZED")]
    Unauthorized,
    /// ## UserRevokeGroupError::DatabaseConnection
    ///
    /// This means that a server-side error occured while trying to do this operation, possibly
    /// intercepting the connection with database.
    ///
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
