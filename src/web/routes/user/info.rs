#[actix_web::get("/me")]
pub async fn controller(
    req: actix_web::HttpRequest,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserInfoParams as Params,
        errors::use_case::UserInfoError as Error
    };
    
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    return match UsersUseCase::info(Params { token, encryption_key: config.jwt.encryption_key.clone() }, &*client.into_inner()).await {
        Ok(user) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(
                to_string(&user).unwrap()
            ),
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::Unauthorized().body("INVALID_TOKEN"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION"),
        }
    };
}
