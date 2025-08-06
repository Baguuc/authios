#[actix_web::patch("/user/pwd")]
pub async fn update_pwd_route(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use authios_application::{
        UsersUseCase,
        use_cases::user::update_pwd::UserUpdatePwdError as Error
    };

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().into()
    };

    return match UsersUseCase::update_pwd(&token, &config.jwt.encryption_key, &body.pwd, &*client).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body(error.to_string()),
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::CannotHash => HttpResponse::InternalServerError().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    }
}

#[derive(serde::Deserialize)]
struct RequestBody {
    pwd: String
}
