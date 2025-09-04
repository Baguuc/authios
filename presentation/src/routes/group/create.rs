#[actix_web::post("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        GroupsUseCase,
        use_cases::group::create::GroupCreateError as Error
    };
    use authios_domain::{GroupCreateParamsBuilder, AuthParamsBuilder};
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

    let params = GroupCreateParamsBuilder::new()
        .set_name(body.name.clone())
        .set_auth(auth_params)
        .build()
        .unwrap();

    return match GroupsUseCase::create(params, &*client).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String
}
