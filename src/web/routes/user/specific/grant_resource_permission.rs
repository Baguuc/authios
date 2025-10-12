#[actix_web::post("/permissions/resource")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    body: actix_web::web::Json<Body>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserGrantResourcePermissionParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserGrantResourcePermissionResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        user_id: &path.user_id,
        service_id: &body.service_id,
        resource_type: &body.resource_type,
        resource_id: &body.resource_id,
        permission_name: &body.permission_name,
        password: &root_password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::grant_resource_permission(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32
}

#[derive(serde::Deserialize)]
struct Body {
    service_id: String,
    resource_type: String,
    resource_id: i32,
    permission_name: String
}
