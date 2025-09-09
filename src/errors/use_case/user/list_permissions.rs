#[derive(thiserror::Error, Debug)]
pub enum UserListPermissionsError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
