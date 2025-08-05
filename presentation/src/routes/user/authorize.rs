#[actix_web::get("/authorize/{permission_name}")]
pub async fn authorize_route(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::UsersUseCase;
    use actix_web::HttpResponse;

    let client = client.into_inner();
    
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("")
    };

    match UsersUseCase::check_permission(&token, &config.jwt.encryption_key, &path.permission_name, &*client).await {
        Ok(true) => return HttpResponse::Ok().body(""),
        Ok(false) | Err(_) => return HttpResponse::Unauthorized().body("")
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    permission_name: String
}
