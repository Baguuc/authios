#[actix_web::patch("/user/pwd")]
pub async fn update_pwd_route(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use authios_application::UsersUseCase;

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().into()
    };

    match UsersUseCase::update_pwd(&token, &config.jwt.encryption_key, &body.pwd, &*client).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(_) => return HttpResponse::InternalServerError().into()
    }
}

#[derive(serde::Deserialize)]
struct RequestBody {
    pwd: String
}
