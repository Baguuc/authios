#[actix_web::post("/permissions/service")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    body: actix_web::web::Json<Body>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserGrantServicePermissionParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserGrantServicePermissionResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        user_id: &path.user_id,
        service_id: &body.service_id,
        password: &root_password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::grant_service_permission(params, &mut *database_client)
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
    service_id: String
}
