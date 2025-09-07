#[derive(thiserror::Error, Debug)]
pub enum GroupGrantPermissionError {
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
