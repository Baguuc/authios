#[actix_web::get("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    use crate::{
        use_cases::UsersUseCase,
        params::UserInfoParamsBuilder as ParamsBuilder,
        errors::UserInfoError as Error
    };
    
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    let params = ParamsBuilder::new()
        .set_token(token)
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();

    return match UsersUseCase::info(params, &*client.into_inner()).await {
        Ok(user) => HttpResponse::Ok().content_type(ContentType::json()).body(to_string(&user).unwrap()),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body(error.to_string()),
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        }
    };
}
