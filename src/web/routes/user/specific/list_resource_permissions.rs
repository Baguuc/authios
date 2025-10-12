#[actix_web::get("/permissions/resource/{service_id}/{resource_type}")]
pub async fn controller(
    password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserListResourcePermissionsParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserListResourcePermissionsResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        id: &path.user_id,
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        page_number: &query.page_number.unwrap_or(0),
        password: &password.0,
        root_password: &config.root.password
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
    user_id: i32,
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
