#[actix_web::get("/{service_id}/{resource_type}/{resource_id}/users")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    query: actix_web::web::Query<Query>,
    database_client: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use crate::params::use_case::ResourcePermissionListUsersParams as Params;
    use crate::use_cases::ResourcePermissionUseCase as UseCase;
    use crate::web::responses::ResourcePermissionListUsersResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        resource_id: &path.resource_id,
        password: &root_password.0,
        root_password: &config.root.password,
        page_number: &query.page_number.unwrap_or(0)
    };

    let response: Response = UseCase::list_users(params, &mut *database_client)
        .await
        .into();

    response.partialize_ok(
        query.get_page_number.unwrap_or(true),
        query.get_id.unwrap_or(true),
        query.get_login.unwrap_or(true),
        query.get_password_hash.unwrap_or(true)
    )
        .into()
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String,
    resource_id: String
}

#[derive(serde::Deserialize)]
struct Query {
    page_number: Option<u32>,
    get_page_number: Option<bool>,
    get_id: Option<bool>,
    get_login: Option<bool>,
    get_password_hash: Option<bool>
}
