#[actix_web::get("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    use authios_application::{
        UsersUseCase,
        use_cases::user::retrieve_from_token::UserRetrieveFromTokenError as Error
    };
    
    let client = client.into_inner();

    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("NO_TOKEN")
    };

    return match UsersUseCase::retrieve_from_token(&token, &config.jwt.encryption_key.clone(), &*client).await {
        Ok(user) => HttpResponse::Ok().content_type(ContentType::json()).body(to_string(&user).unwrap()),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body(error.to_string()),
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        }
    };
}
