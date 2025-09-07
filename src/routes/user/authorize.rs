#[actix_web::get("/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::UsersUseCase,
        params::UserAuthorizeParamsBuilder as ParamsBuilder,
        errors::UserAuthorizeError as Error
    };
    use actix_web::HttpResponse;

    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let params = ParamsBuilder::new()
        .set_token(token)
        .set_encryption_key(config.jwt.encryption_key.clone())
        .set_permission_name(path.permission_name.clone())
        .build()
        .unwrap();

    return match UsersUseCase::authorize(params, &*client.into_inner()).await {
        Ok(true) => HttpResponse::Ok().into(),
        Ok(false) => HttpResponse::Unauthorized().into(),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body(error.to_string()),
            Error::UserNotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::PermissionNotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    permission_name: String
}
