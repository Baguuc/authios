#[derive(thiserror::Error, Debug)]
pub enum UserUpdatePwdError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("CANNOT_HASH")]
    CannotHash,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
