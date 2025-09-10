#[derive(thiserror::Error, Debug)]
pub enum GroupRevokePermissionError {
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
