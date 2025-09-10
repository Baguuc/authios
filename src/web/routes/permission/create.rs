#[actix_web::post("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::PermissionsUseCase,
        params::use_case::PermissionCreateParamsBuilder as ParamsBuilder,
        errors::use_case::PermissionCreateError as Error
    };
    use actix_web::HttpResponse;

    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let params = ParamsBuilder::new()
        .set_name(body.name.clone())
        .set_token(token)
        .set_encryption_key(config.jwt.encryption_key.clone())
        .build()
        .unwrap();

    return match PermissionsUseCase::create(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body("ALREADY_EXIST"),
            Error::RootGroupNotFound => HttpResponse::Conflict().body("ROOT_GROUP_NOT_FOUND"),
            Error::Unauthorized => HttpResponse::Unauthorized().body("UNAUTHORIZED"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String
}
