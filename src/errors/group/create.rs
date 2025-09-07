#[derive(thiserror::Error, Debug)]
pub enum GroupCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
