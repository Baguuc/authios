// general
pub mod utils;
// domain
pub mod models;
pub mod params;
pub mod errors;
// application
pub mod repositories;
pub mod use_cases;
pub mod migrations;
// presentation
pub mod routes;
pub mod config;
pub mod cli;

#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = crate::cli::MainCli::parse();
    cli.execute();
}
