#[derive(thiserror::Error, Debug)]
pub enum GroupGrantPermissionError {
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
