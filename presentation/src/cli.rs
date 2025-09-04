use crate::prelude::*;

#[derive(clap::Parser)] // requires `derive` feature
#[command(name = "authios")]
#[command(bin_name = "authios")]
#[command(about = "A simple API for managing users and permissions in closed systems", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(Args),
    #[command(about = "Run migrations on the database")]
    Migrate(Args)
}

#[derive(clap::Args, Clone)]
pub struct Args {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    pub fn execute(self) {
        use futures::executor::block_on;

        match self {
            Self::Run(args) => { block_on(run(args)); },
            Self::Migrate(args) => { block_on(migrate(args)); }
        };
    }
}

async fn run(args: Args) {
    use colored::Colorize;
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;
    use clin::components::{success, error, header};
    use authios_application::{repositories::{PermissionsRepository, GroupsRepository, GroupPermissionsRepository, UsersRepository, UserGroupsRepository}, utils::hash_password};
    use crate::config::Config;
    use crate::error::error_if_necessary;
    
    migrate(args.clone()).await;
    println!("");
    
    header("Running web server");

    let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./authios.json"))));
    
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
    let server = HttpServer::new(move || {
        let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./authios.json"))));
        let pool = error_if_necessary(block_on(create_pool(config.database.clone())));

        {
            let _ = block_on(PermissionsRepository::insert(&String::from("authios:root:read"), &pool));
            let _ = block_on(PermissionsRepository::insert(&String::from("authios:root:write"), &pool));
            
            let _ = block_on(GroupsRepository::insert(&String::from("authios:root"), &pool));
            
            let _ = block_on(GroupPermissionsRepository::insert(&String::from("authios:root"), &String::from("authios:root:read"), &pool));
            let _ = block_on(GroupPermissionsRepository::insert(&String::from("authios:root"), &String::from("authios:root:write"), &pool));

            let _ = block_on(UsersRepository::insert(&String::from("root"), &hash_password(config.root.pwd.clone()).expect("Cannot hash password"), &pool));
            
            let _ = block_on(UserGroupsRepository::insert(&String::from("authios:root:write"), &String::from("authios:root"), &pool));
        };
        
        App::new()
            .app_data(Data::new(pool))
            .app_data(Data::new(config.clone()))
            .service(crate::routes::user::scope())
            .service(crate::routes::permission::scope())
    });

    let binded_server = match server.bind(("0.0.0.0", config.port.clone())) {
        Ok(server) => server,
        Err(_) => {
            error("Cannot bind to port", config.port);
            
            std::process::exit(1);
        }
    };

    let _ = binded_server.run().await;
}

async fn migrate(args: Args) {
    use clin::components::{header,success};
    use crate::config::Config;
    use crate::error::error_if_necessary;
    
    let config = error_if_necessary(Config::read(args.config.unwrap_or(String::from("./authios.json"))));
    let pool = error_if_necessary(create_pool(config.database.clone()).await);
     
    header("Migrating database");
    
    error_if_necessary(authios_application::utils::migrate(&pool).await);
    success("Migrated");
}

async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
