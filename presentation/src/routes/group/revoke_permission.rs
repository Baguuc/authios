#[actix_web::delete("/{name}/permissions/{permission_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        GroupsUseCase,
        use_cases::group::revoke_permission::GroupRevokePermissionError as Error
    };
    use authios_domain::GroupRevokePermissionParamsBuilder as ParamsBuilder;
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
        .set_permission_name(body.permission_name.clone())
        .set_group_name(body.name.clone())
        .build()
        .unwrap();

    return match GroupsUseCase::revoke(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::NotAddedYet | Error::GroupNotExist | Error::PermissionNotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    name: String,
    permission_name: String,
}
