pub mod commands;

#[derive(clap::Parser)]
#[command(name = "authios")]
#[command(bin_name = "authios")]
#[command(about = "A simple API for managing users and permissions in closed systems", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(CliArgs),
    #[command(about = "Run migrations on the database")]
    Migrate(CliArgs),
    #[command(about = "Init the root user and all mandatory data needed to run the API.")]
    Init(CliArgs)
}

#[derive(clap::Args, Clone)]
pub struct CliArgs {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    pub async fn run() {
        use clap::Parser;

        let cli = Self::parse();
        cli.execute().await;
    }

    pub async fn execute(self) {
        match self {
            Self::Run(args) => { commands::run(args).await; },
            Self::Migrate(args) => { commands::migrate(args).await; },
            Self::Init(args) => { commands::init(args).await; }
        };
    }
}
