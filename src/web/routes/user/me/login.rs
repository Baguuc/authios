#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::UserLoginParams as Params;
    use crate::use_cases::UserUseCase as UseCase;
    use crate::web::responses::UserLoginResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        jwt_encryption_key: &config.jwt.encryption_key,
        login: &body.login, 
        password: &body.password 
    };

    let response: Response = UseCase::login(params, &mut *database_client)
        .await
        .into();
    
    response.into()
}

#[derive(serde::Deserialize)]
struct Body {
    login: String,
    password: String
}
