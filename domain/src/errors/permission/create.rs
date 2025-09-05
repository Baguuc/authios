#[derive(thiserror::Error, Debug)]
pub enum PermissionCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
