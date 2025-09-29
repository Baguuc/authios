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
    
    use params::use_case::UserLoginParams as LParams;
    use params::use_case::UserUpdateParams as UParams;
    use params::use_case::UserRetrieveFromTokenParams as RParams;
    use use_cases::UserUseCase;

    /*let result = UserUseCase::login(LParams { login: &"bgc".to_string(), password: &"321".to_string(), jwt_encryption_key: &config.jwt.encryption_key }, &database_client).await;
    println!("{:?}", result);*/

    let result = UserUseCase::retrieve_from_token(RParams { token: &"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc1OTc3MjIxMH0.rlo2nf-T8BXdMbTBPdF2qzShUNOQ0NSL2KOvsbI0roA".to_string(), jwt_encryption_key: &config.jwt.encryption_key }, &database_client).await;
    println!("{:?}", result);
    
    let result = UserUseCase::update(UParams { token: &"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc1OTc3MjIxMH0.rlo2nf-T8BXdMbTBPdF2qzShUNOQ0NSL2KOvsbI0roA".to_string(), jwt_encryption_key: &config.jwt.encryption_key, new_login: Some("bgc".to_string()), new_password: None }, &database_client).await;
    println!("{:?}", result);
    
    let result = UserUseCase::update(UParams { token: &"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc1OTc3MjIxMH0.rlo2nf-T8BXdMbTBPdF2qzShUNOQ0NSL2KOvsbI0roA".to_string(), jwt_encryption_key: &config.jwt.encryption_key, new_login: None, new_password: Some("321".to_string()) }, &database_client).await;
    println!("{:?}", result);
}
