#[actix_web::post("")]
pub async fn controller( req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::use_case::GroupCreateParams as Params,
        errors::use_case::GroupCreateError as Error,
        utils::web::retrieve_token_from_request
    };
    use actix_web::HttpResponse;

    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    return match GroupsUseCase::create(Params { name: body.name.clone(), encryption_key: config.jwt.encryption_key.clone(), token }, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body("ALREADY_EXIST"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String
}
