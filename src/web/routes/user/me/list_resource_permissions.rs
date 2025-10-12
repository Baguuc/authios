#[actix_web::get("/permissions/resource/{service_id}/{resource_type}")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    path: actix_web::web::Path<Path>,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::LoggedUserListResourcePermissionsParams as Params;
    use crate::use_cases::LoggedUserUseCase as UseCase;
    use crate::web::responses::LoggedUserListResourcePermissionsResponse as Response;

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
        page_number: &query.page_number.unwrap_or(0)
    };

    let response: Response = UseCase::list_resource_permissions(params, &mut *database_client)
        .await
        .into();

    response.partialize_ok(
        query.get_page_number.unwrap_or(true),
        query.get_total_page_count.unwrap_or(true),
        query.get_service_id.unwrap_or(true),
        query.get_resource_type.unwrap_or(true),
        query.get_resource_id.unwrap_or(true),
        query.get_permission_names.unwrap_or(true)
    )
        .into()
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String
}

#[derive(serde::Deserialize)]
struct Query {
    page_number: Option<u32>,
    get_page_number: Option<bool>,
    get_total_page_count: Option<bool>,
    get_service_id: Option<bool>,
    get_resource_type: Option<bool>,
    get_resource_id: Option<bool>,
    get_permission_names: Option<bool>
}
