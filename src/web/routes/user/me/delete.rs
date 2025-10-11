#[actix_web::delete("")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::UserDeleteParams as Params;
    use crate::use_cases::UserUseCase as UseCase;
    use crate::web::responses::UserDeleteResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key
    };

    let response: Response = UseCase::delete(params, &mut *database_client)
        .await
        .into();
    
    response.into()
}
