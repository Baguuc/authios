#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use authios_application::{
        UsersUseCase,
        use_cases::user::login::UserLoginError as Error
    };
    
    return match UsersUseCase::login(&body.login, &body.pwd, config.jwt.encryption_key.clone(), &*client.into_inner()).await {
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
