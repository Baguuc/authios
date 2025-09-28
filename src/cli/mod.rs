pub mod commands;

/// # MainCli
///
/// defines the structure of the cli arguments and flags of the program.
///
#[derive(clap::Parser)]
#[command(name = "authios")]
#[command(bin_name = "authios")]
#[command(about = "A simple API for managing users and permissions in closed systems", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(CliFlags)
}

/// # CliFlags
///
/// defines the structure of cli flags of the program.
///
#[derive(clap::Args, Clone)]
pub struct CliFlags {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    /// ## MainCli::run
    /// 
    /// parses the CLI arguments and flags and runs the program.
    ///
    pub async fn run() {
        use clap::Parser;

        let cli = Self::parse();
        cli.execute().await;
    }

    /// ## MainCli::run
    /// 
    /// runs the program with already scraped arguments and flags.
    ///
    pub async fn execute(self) {
        match self {
            Self::Run(args) => { commands::run(args).await; }
        };
    }
}
