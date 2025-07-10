pub mod error;
pub mod prelude;
pub mod routes;
pub mod config;
pub mod cli;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    let cli = crate::cli::MainCli::parse();
    cli.execute();

    return Ok(());
}
