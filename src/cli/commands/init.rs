/// # init::command
///
/// definition of the 'init' cli command, that is initializing all necessary data for the program
/// to function.
/// Uses [crate::use_cases::UsersUseCase::init_root] method.
///
pub async fn command(args: crate::cli::CliFlags) {
    use clin::components::{success, header};
    use crate::config::Config;
    use crate::utils::{
        database::create_pool,
        error::error_if_necessary
    };
    use crate::use_cases::UsersUseCase;
    use crate::params::use_case::UserInitRootParams as Params;
    
    header("Parsing the config");
    let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./authios.json"))));
    let pool = error_if_necessary(create_pool(config.database).await);

    header("Initializing the root account");
    
    let params = Params {
        pwd: config.root.pwd
    };
    
    error_if_necessary(UsersUseCase::init_root(params, &pool).await);

    success("done");
}
