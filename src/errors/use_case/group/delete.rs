#[derive(thiserror::Error, Debug)]
pub enum GroupDeleteError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
