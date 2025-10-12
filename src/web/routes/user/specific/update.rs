#[actix_web::patch("")]
pub async fn controller(
    path: actix_web::web::Path<Path>,
    body: actix_web::web::Json<Body>,
    password: crate::web::extractors::RootPasswordExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserUpdateParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserUpdateResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        id: &path.user_id,
        password: &password.0, 
        root_password: &config.root.password, 
        new_login: &body.login,
        new_password: &body.password
    };

    let response: Response = UseCase::update(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32
}

#[derive(serde::Deserialize)]
struct Body {
    login: Option<String>,
    password: Option<String>
}
