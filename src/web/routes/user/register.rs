#[actix_web::post("")]
pub async fn controller(
    body: actix_web::web::Json<RequestBody>,
    client: actix_web::web::Data<sqlx::postgres::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::{
        use_cases::UsersUseCase,
        params::use_case::UserRegisterParams as Params,
        errors::use_case::UserRegisterError as Error
    };
    
    return match UsersUseCase::register(Params { login: body.login.clone(), pwd: body.pwd.clone() }, &*client.into_inner()).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(error) => match error {
            Error::AlreadyExist => HttpResponse::Conflict().body("ALREADY_EXIST"),
            Error::CannotHashPassword => HttpResponse::BadRequest().body("CANNOT_HASH_PASSWORD"),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body("DATABASE_CONNECTION"),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    login: String,
    pwd: String
}
