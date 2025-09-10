#[actix_web::delete("/{name}/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::use_case::GroupRevokePermissionParamsBuilder as ParamsBuilder,
        errors::use_case::GroupRevokePermissionError as Error
    };
    use actix_web::HttpResponse;

    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    let params = ParamsBuilder::new()
        .set_permission_name(body.permission_name.clone())
        .set_group_name(body.name.clone())
        .set_token(token)
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();

    return match GroupsUseCase::revoke(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotAddedYet => HttpResponse::Conflict().body("NOT_ADDED_YET"),
            Error::GroupNotFound => HttpResponse::Conflict().body("GROUP_NOT_FOUND"),
            Error::PermissionNotFound => HttpResponse::Conflict().body("PERMISSION_NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String,
    permission_name: String,
}
