#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::{
        use_cases::UsersUseCase,
        params::UserRegisterParamsBuilder as ParamsBuilder,
        errors::use_case::UserRegisterError as Error
    };
    
    let params = ParamsBuilder::new()
        .set_login(body.login.clone())
        .set_pwd(body.pwd.clone())
        .build()
        .unwrap();
    
    return match UsersUseCase::register(params, &*client.into_inner()).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body(error.to_string()),
            Error::CannotHashPassword => HttpResponse::BadRequest().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
