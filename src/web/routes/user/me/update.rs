#[actix_web::patch("")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    token: crate::web::extractors::TokenExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserUpdateParams as Params;
    use crate::errors::use_case::UserUpdateError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key,
        new_login: &body.login,
        new_password: &body.password
    };

    match UseCase::update(params, &mut *database_client).await {
        Ok(_) => HttpResponse::Ok()
            .json(json!({ "code": "ok" })),
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
        }
    }
}

#[derive(serde::Deserialize)]
struct Body {
    login: Option<String>,
    password: Option<String>
}
