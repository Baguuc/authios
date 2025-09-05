#[derive(thiserror::Error, Debug)]
pub enum GroupDeleteError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
