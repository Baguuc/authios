#[actix_web::patch("")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    token: crate::web::extractors::TokenExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::LoggedUserUpdateParams as Params;
    use crate::use_cases::LoggedUserUseCase as UseCase;
    use crate::web::responses::LoggedUserUpdateResponse as Response;

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

    let response: Response = UseCase::update(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Body {
    login: Option<String>,
    password: Option<String>
}
