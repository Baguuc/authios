#[derive(thiserror::Error, Debug)]
pub enum PermissionDeleteError {
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
