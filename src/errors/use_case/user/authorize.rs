#[derive(thiserror::Error, Debug)]
pub enum UserAuthorizeError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
