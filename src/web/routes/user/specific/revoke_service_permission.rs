#[actix_web::delete("/permissions/service/{service_id}")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserRevokeServicePermissionParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserRevokeServicePermissionResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        user_id: &path.user_id,
        service_id: &path.service_id,
        password: &root_password.0,
        root_password: &config.root.password
    };

    let response: Response = UseCase::revoke_service_permission(params, &mut *database_client)
        .await
        .into();
    
    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32,
    service_id: String
}
