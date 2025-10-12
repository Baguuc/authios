#[actix_web::get("")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::LoggedUserInfoParams as Params;
    use crate::use_cases::LoggedUserUseCase as UseCase;
    use crate::web::responses::LoggedUserInfoResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key
    };
    
    let response: Response = UseCase::info(params, &mut *database_client)
        .await
        .into();

    response.partialize_ok(
        query.get_id.unwrap_or(true),
        query.get_login.unwrap_or(true),
        query.get_password_hash.unwrap_or(true)
    )
        .into()
}

#[derive(serde::Deserialize)]
struct Query {
    pub get_id: Option<bool>,
    pub get_login: Option<bool>,
    pub get_password_hash: Option<bool>
}
