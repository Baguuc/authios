#[actix_web::post("/{name}/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::use_case::GroupGrantPermissionParams as Params,
        errors::use_case::GroupGrantPermissionError as Error,
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

    return match GroupsUseCase::grant_permission(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyAdded => HttpResponse::Conflict().body("ALREADY_ADDED"),
            Error::GroupNotFound => HttpResponse::Conflict().body("GROUP_NOT_FOUND"),
            Error::PermissionNotFound => HttpResponse::Conflict().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String,
    permission_name: String,
}
