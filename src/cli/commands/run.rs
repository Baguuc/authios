pub async fn command(args: crate::cli::CliArgs) {
    use colored::Colorize;
    use clin::components::{success, error, header};
    use crate::config::Config;
    use crate::utils::error::error_if_necessary;
    
    header("Parsing the config");
    let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./authios.json"))));
    
    header("Running the web server");
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
    match crate::web::run_api(config.clone()).await {
        Ok(_) => (),
        Err(err) => { error(format!("Cannot start server on port {}.", config.port), err); }
    };
}
