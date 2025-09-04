pub mod create;
pub mod delete;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/groups")
        .service(create::controller)
        .service(delete::controller)
}
