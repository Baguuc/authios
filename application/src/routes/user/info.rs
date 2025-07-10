#[actix_web::get("/user")]
pub async fn info_route(
    req: actix_web::HttpRequest,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde_json::to_string;
    use crate::models::User;
    
    let client = client.into_inner();

    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("")
    };

    let user = match User::from_token(&token, &config.jwt.encryption_key.clone(), &*client).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().body("")
    };

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&user).unwrap());
}
