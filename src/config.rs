#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
    pub root: RootConfig
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct JWTConfig {
    pub encryption_key: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct RootConfig {
    pub pwd: String
}

impl Config {
    pub fn read(path: String) -> Result<Self, ConfigReadError> {
        use std::fs::read_to_string;
        use serde_json::from_str;

        let content = read_to_string(path)?;
        let parsed = from_str::<Self>(&content)?;

        return Ok(parsed);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigReadError {
    #[error("Wrong config format (not JSON)")]
    Serde(#[from] serde_json::error::Error),
    
    #[error("Cannot read the config")]
    FS(#[from] std::io::Error),
}
