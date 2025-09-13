#[actix_web::get("/me/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserAuthorizeParams as Params,
        errors::use_case::UserAuthorizeError as Error,
        utils::web::retrieve_token_from_request
    };
    use actix_web::HttpResponse;

    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    let params = Params {
        encryption_key: config.jwt.encryption_key.clone(),
        permission_name: path.permission_name.clone(),
        token
    };

    return match UsersUseCase::authorize(params, &*client.into_inner()).await {
        Ok(true) => HttpResponse::Ok().into(),
        Ok(false) => HttpResponse::Unauthorized().into(),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body("INVALID_TOKEN"),
            Error::PermissionNotFound => HttpResponse::Conflict().body("PERMISSION_NOT_FOUND"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    permission_name: String
}
