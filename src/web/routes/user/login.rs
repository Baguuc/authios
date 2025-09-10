#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserLoginParamsBuilder as ParamsBuilder,
        errors::use_case::UserLoginError as Error
    };
    
    let params = ParamsBuilder::new()
        .set_login(body.login.clone())
        .set_pwd(body.pwd.clone())
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();
    
    return match UsersUseCase::login(params, &*client.into_inner()).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(error) => match error {
            Error::UserNotFound => HttpResponse::Conflict().body("USER_NOT_FOUND"),
            Error::WrongPassword => HttpResponse::Unauthorized().body("WRONG_PASSWORD"),
            Error::CannotGenerateToken => HttpResponse::InternalServerError().body("CANNOT_GENERATE_TOKEN"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION"),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
