#[actix_web::get("")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserInfoParams as Params;
    use crate::errors::use_case::UserInfoError as Error;
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

    match UseCase::info(params, &mut *database_client).await {
        Ok(user) => {
            let response = Response {
                user: ResponseUser {
                    id: if query.get_id.unwrap_or(true)
                        { Some(user.id) } else { None },
                    login: if query.get_login.unwrap_or(true)
                        { Some(user.login.clone()) } else { None },
                    password_hash: if query.get_password_hash.unwrap_or(true)
                        { Some(user.password_hash.clone()) } else { None }
                }
            };

            HttpResponse::Ok()
                .json(response)
        },
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
        }
    }
}

#[derive(serde::Serialize)]
struct Response {
    user: ResponseUser
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct ResponseUser {
    id: Option<i32>,
    login: Option<String>,
    password_hash: Option<String>
}

#[derive(serde::Deserialize)]
struct Query {
    pub get_id: Option<bool>,
    pub get_login: Option<bool>,
    pub get_password_hash: Option<bool>
}
