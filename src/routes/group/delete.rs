#[actix_web::delete("/{name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Path<RequestPath>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use crate::{
        use_cases::GroupsUseCase,
        params::GroupDeleteParamsBuilder as ParamsBuilder,
        errors::GroupDeleteError as Error
    };
    use actix_web::HttpResponse;
    
    let pwd = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    if pwd != config.root.pwd {
        return HttpResponse::Unauthorized().body("UNAUTHORIZED");
    }

    let params = ParamsBuilder::new()
        .set_name(body.name.clone())
        .build()
        .unwrap();

    return match GroupsUseCase::delete(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestPath {
    name: String
}

