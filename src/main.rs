// general
pub mod utils;
// domain
pub mod models;
pub mod params;
pub mod errors;
// application
pub mod repositories;
pub mod use_cases;
// presentation
pub mod web;
pub mod config;
pub mod cli;

#[tokio::main]
async fn main() {
    crate::cli::MainCli::run().await;
}
