#[actix_web::delete("/{name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::use_case::GroupDeleteParams as Params,
        errors::use_case::GroupDeleteError as Error
    };
    use actix_web::HttpResponse;
    
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    return match GroupsUseCase::delete(Params { name: body.name.clone(), encryption_key: config.jwt.encryption_key.clone(), token }, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotFound => HttpResponse::NotFound().body("NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    name: String
}

