/// Represents one of errors that can occur while running the API's HTTP server with authios::web::run_api function.
///
#[derive(thiserror::Error, Debug)]
pub enum ServerRunError {
    /// ## RunApiError::Server
    ///
    /// This means that the server cannot be created for some reason.
    ///
    #[error("Error running server: {0}")]
    Server(#[from] actix_web::Error),
    /// ## RunApiError::DatabaseConnection
    ///
    /// This means that the database connection cannot be established with the connection data
    /// provided.
    ///
    #[error("Error connection to database: {0}")]
    DatabaseConnection(#[from] sqlx::Error),
    /// ## RunApiError::StdIO
    ///
    /// This means that the server cannot be binded on the provided port.
    ///
    #[error("Error binding server: {0}")]
    StdIO(#[from] std::io::Error),
}
