mod register;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(register::controller)
}
