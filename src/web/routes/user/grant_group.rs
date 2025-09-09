#[actix_web::post("/{login}/groups/{group_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserGrantGroupParamsBuilder as ParamsBuilder,
        errors::use_case::UserGrantGroupError as Error
    };
    use actix_web::HttpResponse;

    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let params = ParamsBuilder::new()
        .set_group_name(body.group_name.clone())
        .set_user_login(body.login.clone())
        .set_token(token)
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();

    return match UsersUseCase::grant_group(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyAdded | Error::UserNotExist | Error::GroupNotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    group_name: String,
}
