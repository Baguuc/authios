#[actix_web::delete("/{service_id}")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    body: actix_web::web::Json<Path>,
    database_client: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use crate::params::use_case::ServicePermissionDeleteParams as Params;
    use crate::use_cases::ServicePermissionUseCase as UseCase;
    use crate::web::responses::ServicePermissionDeleteResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        service_id: &body.service_id,
        password: &root_password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::delete(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String
}
