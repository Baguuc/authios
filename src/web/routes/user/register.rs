#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<Body>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserRegisterParams as Params;
    use crate::errors::use_case::UserRegisterError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params { 
        login: &body.login, 
        password: &body.password 
    };

    match UseCase::register(params, &mut *database_client).await {
        Ok(_) => HttpResponse::Created()
            .json(json!({ "code": "ok" })),

        Err(error) => match error {
            Error::AlreadyExists => HttpResponse::Conflict()
                .json(serde_json::json!({ "code": "already_exists" }))
        }
    }
}

#[derive(serde::Deserialize)]
struct Body {
    login: String,
    password: String
}
