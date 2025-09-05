#[actix_web::post("/{login}/groups/{group_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>,
    config: actix_web::web::Data<crate::config::Config>,
) -> impl actix_web::Responder {
    use authios_application::{
        UsersUseCase,
        use_cases::user::grant_group::UserGrantGroupError as Error
    };
    use authios_domain::UserGrantGroupParamsBuilder as ParamsBuilder;
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
        .set_group_name(body.group_name.clone())
        .set_user_login(body.login.clone())
        .build()
        .unwrap();

    return match UsersUseCase::grant_group(params, &*client.into_inner()).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::AlreadyAdded | Error::UserNotExist | Error::GroupNotExist => HttpResponse::Conflict().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    group_name: String,
}
