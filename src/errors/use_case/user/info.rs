#[derive(thiserror::Error, Debug)]
pub enum UserInfoError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
