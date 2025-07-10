#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    
    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;
