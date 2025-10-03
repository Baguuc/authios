mod me;
mod specific;
mod register;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(register::controller)
        .service(specific::scope())
        .service(me::scope())
}
