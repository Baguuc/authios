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
    
    use params::repository::UserResourcePermissionRetrieveParams as Params;

    let result = crate::repositories::UserResourcePermissionRepository::retrieve(Params {
        user_id: &1,
        service_id: &String::from("hostios"),
        resource_type: &String::from("vault"),
        resource_id: &111
    }, &database_client).await;
    println!("{:?}", result);
    
    use params::repository::UserResourcePermissionInsertParams as IParams;
    let result = crate::repositories::UserResourcePermissionRepository::insert(IParams {
        user_id: &1,
        resource_permission_id: &1,
        resource_id: &111
    }, &database_client).await;
    println!("{:?}", result);
    
    let result = crate::repositories::UserResourcePermissionRepository::retrieve(Params {
        user_id: &1,
        service_id: &String::from("hostios"),
        resource_type: &String::from("vault"),
        resource_id: &111
    }, &database_client).await;
    println!("{:?}", result);
}
