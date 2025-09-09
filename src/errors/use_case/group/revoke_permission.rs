#[derive(thiserror::Error, Debug)]
pub enum GroupRevokePermissionError {
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
