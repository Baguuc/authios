#[actix_web::patch("/pwd")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserUpdatePwdParamsBuilder as ParamsBuilder,
        errors::use_case::UserUpdatePwdError as Error
    };
    
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let params = ParamsBuilder::new()
        .set_token(token)
        .set_new_pwd(body.pwd.clone())
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();

    return match UsersUseCase::update_pwd(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body("INVALID_TOKEN"),
            Error::CannotHash => HttpResponse::InternalServerError().body("CANNOT_HASH_PWD"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION"),
        } 
    }
}

#[derive(serde::Deserialize)]
struct RequestBody {
    pwd: String
}
