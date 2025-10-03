#[actix_web::delete("/{service_id}/{resource_type}/{permission_name}")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    body: actix_web::web::Json<Path>,
    database_client: actix_web::web::Data<sqlx::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::ResourcePermissionDeleteParams as Params;
    use crate::errors::use_case::ResourcePermissionDeleteError as Error;
    use crate::use_cases::ResourcePermissionUseCase as UseCase;

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

    match UseCase::delete(params, &mut *database_client).await {
        Ok(_) => HttpResponse::Ok()
            .json(json!({ "msg": "ok" })),

        Err(error) => match error {
            Error::NotFound => HttpResponse::Conflict()
                .json(serde_json::json!({ "msg": "not_found" })),
            
            Error::Unauthorized => HttpResponse::Unauthorized()
                .json(serde_json::json!({ "msg": "wrong_password" }))
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String,
    permission_name: String
}
