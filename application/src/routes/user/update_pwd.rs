#[actix_web::patch("/user/pwd")]
pub async fn update_pwd_route(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>
) -> impl actix_web::Responder {
    use actix_web::HttpResponse;
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

    let pwd = match User::hash_password(body.pwd.to_string()) {
        Ok(pwd) => pwd,
        Err(_) => return HttpResponse::BadRequest().body("")
    };

    match User::update_pwd(&user.login, &pwd, &*client).await {
        Ok(_) => return HttpResponse::Ok().body(""),
        Err(_) => return HttpResponse::InternalServerError().body("")
    }
}

#[derive(serde::Deserialize)]
struct RequestBody {
    pwd: String
}
