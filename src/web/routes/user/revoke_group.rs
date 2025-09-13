#[actix_web::delete("/{login}/groups/{group_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserRevokeGroupParams as Params,
        errors::use_case::UserRevokeGroupError as Error,
        utils::web::retrieve_token_from_request
    };
    use actix_web::HttpResponse;

    let token = match retrieve_token_from_request(req) {
        Some(token) => token,
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    let params = Params { 
        group_name: body.group_name.clone(),
        user_login: body.login.clone(),
        encryption_key: config.jwt.encryption_key.clone(),
        token
    };
    
    return match UsersUseCase::revoke_group(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(error) => match error {
            Error::UserNotFound => HttpResponse::NotFound().body("USER_NOT_FOUND"),
            Error::GroupNotFound => HttpResponse::NotFound().body("GROUP_NOT_FOUND"),
            Error::NotAddedYet => HttpResponse::Conflict().body("NOT_ADDED_YET"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::ServiceUnavailable().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    group_name: String,
}
