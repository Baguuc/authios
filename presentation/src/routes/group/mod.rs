pub mod create;
pub mod delete;
pub mod grant;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/groups")
        .service(create::controller)
        .service(delete::controller)
        .service(grant::controller)
}
