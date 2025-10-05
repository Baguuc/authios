#[actix_web::get("/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name}")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    path: actix_web::web::Path<Path>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::UserCheckResourcePermissionParams as Params;
    use crate::use_cases::UserUseCase as UseCase;
    use crate::web::responses::UserCheckResourcePermissionResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key,
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        resource_id: &path.resource_id,
        permission_name: &path.permission_name
    };

    let response: Response = UseCase::check_resource_permission(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String,
    resource_id: i32,
    permission_name: String
}
