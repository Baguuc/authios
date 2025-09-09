#[derive(thiserror::Error, Debug)]
pub enum UserGrantGroupError {
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
