#[derive(thiserror::Error, Debug)]
pub enum UserRegisterError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("CANNOT_HASH_PASSWORD")]
    CannotHashPassword,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
