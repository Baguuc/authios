#[derive(thiserror::Error, Debug)]
pub enum PermissionCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
