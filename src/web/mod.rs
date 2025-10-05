pub mod routes;
pub mod extractors;

pub async fn run_api(config: crate::config::Config) -> Result<(), crate::errors::web::ServerRunError> {
    use actix_web::web::{
        PathConfig,
        QueryConfig,
        JsonConfig
    };
    use crate::utils::database::create_pool;
    use crate::errors::web::{
        PathDeserializeError,
        QueryDeserializeError,
        JsonDeserializeError
    };

    let port = config.port;
    let pool = create_pool(config.database.clone()).await?;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(
                PathConfig::default().error_handler(|err, _req| PathDeserializeError(err).into())
            )
            .app_data(
                QueryConfig::default().error_handler(|err, _req| QueryDeserializeError(err).into())
            )
            .app_data(
                JsonConfig::default().error_handler(|err, _req| JsonDeserializeError(err).into())
            )
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
