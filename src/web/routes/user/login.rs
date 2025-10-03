#[actix_web::post("/me")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserLoginParams as Params;
    use crate::errors::use_case::UserLoginError as Error;
    use crate::use_cases::UserUseCase as UseCase;

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

    match UseCase::login(params, &mut *database_client).await {
        Ok(token) => HttpResponse::Ok()
            .json(json!({ "token": token })),
        
        Err(error) => match error {
            Error::NotFound => HttpResponse::NotFound()
                .json(json!({ "msg": "not_found" })),

            Error::WrongPassword => HttpResponse::Unauthorized()
                .json(json!({ "msg": "wrong_password" })),
        }
    }
}

#[derive(serde::Deserialize)]
struct Body {
    login: String,
    password: String
}
