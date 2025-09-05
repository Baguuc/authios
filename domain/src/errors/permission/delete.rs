#[derive(thiserror::Error, Debug)]
pub enum PermissionDeleteError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
