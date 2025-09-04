#[actix_web::delete("/{name}/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        GroupsUseCase,
        use_cases::group::revoke::GroupPermissionRevokeError as Error
    };
    use authios_domain::{PermissionRevokeParamsBuilder, AuthParamsBuilder};
    use actix_web::HttpResponse;

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("NO_TOKEN")
    };

    let auth_params = AuthParamsBuilder::new()
        .set_encoding_key(config.jwt.encryption_key.clone())
        .set_token(token)
        .build()
        // won't error
        .unwrap();

    let params = PermissionRevokeParamsBuilder::new()
        .set_name(body.permission_name.clone())
        .set_group_name(body.name.clone())
        .set_auth(auth_params)
        .build()
        .unwrap();

    return match GroupsUseCase::revoke(params, &*client).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotAddedYet | Error::GroupNotExist | Error::PermissionNotExist => HttpResponse::Conflict().body(error.to_string()),
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
