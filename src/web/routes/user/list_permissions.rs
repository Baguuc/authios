#[actix_web::get("/permissions")]
pub async fn controller(
    req: actix_web::HttpRequest,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserListPermissionsParams as Params,
        errors::use_case::UserListPermissionsError as Error,
        utils::web::retrieve_token_from_request
    };
    
    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    return match UsersUseCase::list_permissions(Params { token, encryption_key: config.jwt.encryption_key.clone() }, &*client.into_inner()).await {
        Ok(permissions) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(
                to_string(&permissions).unwrap()
            ),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body("INVALID_TOKEN"),
            Error::DatabaseConnection => HttpResponse::ServiceUnavailable().body("DATABASE_CONNECTION"),
        }
    };
}
