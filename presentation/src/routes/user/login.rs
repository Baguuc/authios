#[actix_web::post("/user")]
pub async fn login_route(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use authios_application::UsersUseCase;
    
    let client = client.into_inner();
    
    let token = match UsersUseCase::login(&body.login, &body.pwd, config.jwt.encryption_key.clone(), &*client).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().into()
    };

    return HttpResponse::Ok()
        .body(token);
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
