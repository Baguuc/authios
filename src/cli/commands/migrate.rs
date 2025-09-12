/// # migrate::command
///
/// definition of the 'migrate' cli command, that is migrating the database to the state necessary for the program
/// to function.
/// Uses [crate::utils::database::migrate] function.
///
pub async fn command(args: crate::cli::CliFlags) {
    use clin::components::{header,success};
    use crate::config::Config;
    use crate::utils::{
        error::error_if_necessary,
        database::{
            migrate,
            create_pool
        }
    };
    
    let config = error_if_necessary(Config::read(args.config.unwrap_or(String::from("./authios.json"))));
    let pool = error_if_necessary(create_pool(config.database.clone()).await);
     
    header("Migrating database");
    
    error_if_necessary(migrate(&pool).await);
    success("Migrated");
}
