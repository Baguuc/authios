#[derive(thiserror::Error, Debug)]
pub enum UserInitRootError {
    #[error("Cannot create permission \"authios:all\". Already exists.")]
    PermissionExists,
    #[error("Cannot create group \"root\". Already exists.")]
    GroupExists,
    #[error("Cannot create user \"root\". Already exists.")]
    UserExists,
    #[error("Cannot hash root user's password from config")]
    CannotHashPassword,
    #[error("Database connection cannot be acquired.")]
    DatabaseConnection
}
