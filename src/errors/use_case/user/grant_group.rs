#[derive(thiserror::Error, Debug)]
pub enum UserGrantGroupError {
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
