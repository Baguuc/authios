#[derive(thiserror::Error, Debug)]
pub enum UserListPermissionsError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
