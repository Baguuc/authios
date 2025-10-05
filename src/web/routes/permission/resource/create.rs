#[actix_web::post("")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    body: actix_web::web::Json<Body>,
    database_client: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use crate::params::use_case::ResourcePermissionCreateParams as Params;
    use crate::use_cases::ResourcePermissionUseCase as UseCase;
    use crate::web::responses::ResourcePermissionCreateResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        service_id: &body.service_id,
        resource_type: &body.resource_type,
        permission_name: &body.permission_name,
        password: &root_password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::create(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Body {
    service_id: String,
    resource_type: String,
    permission_name: String
}
