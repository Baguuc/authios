#[derive(thiserror::Error, Debug)]
pub enum PermissionDeleteError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
