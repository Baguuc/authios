#[actix_web::post("/user")]
pub async fn login_route(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use authios_application::{
        UsersUseCase,
        use_cases::user::login::UserLoginError as Error
    };
    
    let client = client.into_inner();
    
    return match UsersUseCase::login(&body.login, &body.pwd, config.jwt.encryption_key.clone(), &*client).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(error) => match error {
            Error::InvalidCredentials => HttpResponse::Unauthorized().body(error.to_string()),
            Error::NotExist => HttpResponse::Unauthorized().body(error.to_string()),
            Error::CannotGenerateToken => HttpResponse::InternalServerError().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
