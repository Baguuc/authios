#[actix_web::get("/permissions/resource/{service_id}/{resource_type}")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    path: actix_web::web::Path<Path>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserListResourcePermissionsParams as Params;
    use crate::errors::use_case::UserListResourcePermissionsError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key,
        service_id: &path.service_id,
        resource_type: &path.resource_type
    };

    match UseCase::list_resource_permissions(params, &mut *database_client).await {
        Ok(token) => HttpResponse::Ok()
            .json(json!({ "token": token })),
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "msg": "invalid_token" })),
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String
}
