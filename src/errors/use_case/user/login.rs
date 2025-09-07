#[derive(thiserror::Error, Debug)]
pub enum UserLoginError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[error("CANNOT_GENERATE_TOKEN")]
    CannotGenerateToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
