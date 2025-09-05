#[actix_web::delete("/{name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        GroupsUseCase,
        use_cases::group::delete::GroupDeleteError as Error
    };
    use authios_domain::{GroupDeleteParamsBuilder, AuthParamsBuilder};
    use actix_web::HttpResponse;
    
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let auth_params = AuthParamsBuilder::new()
        .set_encoding_key(config.jwt.encryption_key.clone())
        .set_token(token)
        .build()
        // won't error
        .unwrap();

    let params = GroupDeleteParamsBuilder::new()
        .set_name(body.name.clone())
        .set_auth(auth_params)
        .build()
        .unwrap();

    return match GroupsUseCase::delete(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    name: String
}

