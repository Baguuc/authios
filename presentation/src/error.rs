#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
    
    #[error("Serde {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error(transparent)]
    Actix(#[from] actix_web::Error),

    #[error(transparent)]
    Clap(#[from] clap::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),   
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub fn error_if_necessary<T, E: ToString>(r: std::result::Result<T, E>) -> T {
    match r {
        Ok(ok) => return ok,
        Err(err) => {
            clin::components::error("something went wrong", err.to_string());
            std::process::exit(1);
        }
    }
}
