#[actix_web::delete("/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name}")]
pub async fn controller(
    root_password: crate::web::extractors::RootPasswordExtractor,
    path: actix_web::web::Path<Path>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserRevokeResourcePermissionParams as Params;
    use crate::errors::use_case::UserRevokeResourcePermissionError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        user_id: &path.user_id,
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        resource_id: &path.resource_id,
        permission_name: &path.permission_name,
        password: &root_password.0,
        root_password: &config.root.password
    };

    match UseCase::revoke_resource_permission(params, &mut *database_client).await {
        Ok(_) => HttpResponse::Ok()
            .json(json!({ "code": "ok" })),
        
        Err(error) => match error {
            Error::UserNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
            Error::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" })),
            
            Error::NotAddedYet => HttpResponse::Conflict()
                .json(json!({ "code": "not_added_yet" })),
            
            Error::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "wrong_password" }))
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32,
    service_id: String,
    resource_type: String,
    resource_id: i32,
    permission_name: String
}
