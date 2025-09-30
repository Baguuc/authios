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
    //crate::cli::MainCli::run().await;
    let config = crate::config::Config::read("./config.json".to_string()).unwrap();
    let database_client = crate::utils::database::create_pool(config.database).await.unwrap();
    
    use params::use_case::ResourcePermissionDeleteParams as Params;
    use use_cases::ResourcePermissionUseCase;

    /*let result = UserUseCase::login(LParams { login: &"bgc".to_string(), password: &"321".to_string(), jwt_encryption_key: &config.jwt.encryption_key }, &database_client).await;
    println!("{:?}", result);*/

    let params = Params {
        password: &String::from("123"),
        root_password: &config.root.password,
        service_id: &String::from("hostios"),
        resource_type: &String::from("vault"),
        permission_name: &String::from("manage")
    };
    let result = ResourcePermissionUseCase::delete(params, &database_client).await;
    println!("{:?}", result);
}
