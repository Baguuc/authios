mod create;
mod delete;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/service")
        .service(create::controller)
        .service(delete::controller)
}
