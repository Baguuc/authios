#[actix_web::delete("/{name}/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::use_case::GroupRevokePermissionParams as Params,
        errors::use_case::GroupRevokePermissionError as Error,
        utils::web::retrieve_token_from_request
    };
    use actix_web::HttpResponse;

    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };
    
    let params = Params {
        permission_name: body.permission_name.clone(),
        group_name: body.name.clone(),
        encryption_key: config.jwt.encryption_key.clone(),
        token
    };

    return match GroupsUseCase::revoke_permission(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(error) => match error {
            Error::NotAddedYet => HttpResponse::Conflict().body("NOT_ADDED_YET"),
            Error::GroupNotFound => HttpResponse::NotFound().body("GROUP_NOT_FOUND"),
            Error::PermissionNotFound => HttpResponse::NotFound().body("PERMISSION_NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::ServiceUnavailable().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String,
    permission_name: String,
}
