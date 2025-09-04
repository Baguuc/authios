pub mod create;
pub mod delete;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/permissions")
        .service(create::controller)
        .service(delete::controller)
}
