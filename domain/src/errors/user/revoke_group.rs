#[derive(thiserror::Error, Debug)]
pub enum UserRevokeGroupError {
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
