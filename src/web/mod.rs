pub mod routes;
pub mod extractors;

pub async fn run_api(config: crate::config::Config) -> Result<(), crate::errors::web::ServerRunError> {
    use crate::utils::database::create_pool;

    let port = config.port;
    let pool = create_pool(config.database.clone()).await?;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(config.clone()))
            .service(routes::user::scope())
            .service(routes::permission::scope())
    });
    server.bind(("0.0.0.0", port))?
        .run()
        .await?;

    return Ok(());
}
