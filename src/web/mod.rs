pub mod routes;

pub async fn run_api(config: crate::config::Config) -> Result<(), crate::errors::web::RunApiError> {
    use crate::utils::database::create_pool;

    let port = config.port;
    let pool = create_pool(config.database.clone()).await?;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(config.clone()))
            .service(crate::web::routes::user::scope())
            .service(crate::web::routes::permission::scope())
            .service(crate::web::routes::group::scope())
    });
    server.bind(("0.0.0.0", port))?
        .run()
        .await?;

    return Ok(());
}
