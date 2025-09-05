#[derive(thiserror::Error, Debug)]
pub enum UserInfoError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("NotExist")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
