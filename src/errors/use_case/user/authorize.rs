#[derive(thiserror::Error, Debug)]
pub enum UserAuthorizeError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
