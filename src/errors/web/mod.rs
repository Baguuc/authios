#[derive(thiserror::Error, Debug)]
pub enum RunApiError {
    #[error("Error running server: {0}")]
    Server(#[from] actix_web::Error),
    
    #[error("Error connection to database: {0}")]
    DatabaseConnection(#[from] sqlx::Error),
    
    #[error("Error binding server: {0}")]
    StdIO(#[from] std::io::Error),
}
