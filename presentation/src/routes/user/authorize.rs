#[actix_web::get("/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        UsersUseCase,
        use_cases::user::check_permission::UserCheckPermissionError as Error
    };
    use actix_web::HttpResponse;

    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    return match UsersUseCase::check_permission(&token, &config.jwt.encryption_key, &path.permission_name, &*client.into_inner()).await {
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
