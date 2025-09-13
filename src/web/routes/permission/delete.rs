#[actix_web::delete("/{name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::PermissionsUseCase,
        params::use_case::PermissionDeleteParams as Params,
        errors::use_case::PermissionDeleteError as Error,
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

    return match PermissionsUseCase::delete(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotFound => HttpResponse::Conflict().body("NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    name: String
}

