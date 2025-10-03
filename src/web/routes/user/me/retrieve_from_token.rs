#[actix_web::get("")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserRetrieveFromTokenParams as Params;
    use crate::errors::use_case::UserRetrieveFromTokenError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key
    };

    match UseCase::retrieve_from_token(params, &mut *database_client).await {
        Ok(token) => HttpResponse::Ok()
            .json(json!({ "token": token })),
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "msg": "invalid_token" })),
        }
    }
}
