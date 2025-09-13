#[actix_web::post("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::PermissionsUseCase,
        params::use_case::PermissionCreateParams as Params,
        errors::use_case::PermissionCreateError as Error,
        utils::web::retrieve_token_from_request
    };
    use actix_web::HttpResponse;

    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    let params = Params {
        name: body.name.clone(),
        encryption_key: config.jwt.encryption_key.clone(),
        token
    };

    return match PermissionsUseCase::create(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Created().into(),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body("ALREADY_EXIST"),
            Error::RootGroupNotFound => HttpResponse::NotFound().body("ROOT_GROUP_NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::ServiceUnavailable().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String
}
