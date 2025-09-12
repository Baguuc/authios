/// # ConfigReadError
///
/// Represents one of errors that can occur during reading a config using
/// [crate::config::Config::read] method.
///
#[derive(thiserror::Error, Debug)]
pub enum ConfigReadError {
    /// ## ConfigReadError::Serde
    ///
    /// This means that the provided config cannot be deserialized, meaning that it's formatted
    /// wrong or possibly is lacking mandatory fields.
    ///
    #[error("Wrong config format (not JSON)")]
    Serde(#[from] serde_json::error::Error),
    /// ## ConfigReadError::FS
    ///
    /// This means that config file at provided file either cannot be read or the program is not
    /// authorized to read it (for example it's in a folder with root owner).
    #[error("Cannot read the config")]
    FS(#[from] std::io::Error),
}
