#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::AllUserRegisterParams as Params;
    use crate::use_cases::AllUserUseCase as UseCase;
    use crate::web::responses::AllUserRegisterResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params { 
        login: &body.login, 
        password: &body.password 
    };

    let response: Response = UseCase::register(params, &mut *database_client)
        .await
        .into();

    response.into()
}

#[derive(serde::Deserialize)]
struct Body {
    login: String,
    password: String
}
