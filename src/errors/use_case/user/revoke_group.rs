#[derive(thiserror::Error, Debug)]
pub enum UserRevokeGroupError {
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
