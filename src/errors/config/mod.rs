#[derive(thiserror::Error, Debug)]
pub enum ConfigReadError {
    #[error("Wrong config format (not JSON)")]
    Serde(#[from] serde_json::error::Error),
    
    #[error("Cannot read the config")]
    FS(#[from] std::io::Error),
}
