#[actix_web::get("/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name}")]
pub async fn controller(
    password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserCheckResourcePermissionParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserCheckResourcePermissionResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        id: &path.user_id,
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        resource_id: &path.resource_id,
        permission_name: &path.permission_name,
        password: &password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::check_resource_permission(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32,
    service_id: String,
    resource_type: String,
    resource_id: String,
    permission_name: String
}
