mod me;
mod specific;
mod all;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(me::scope())
        .service(specific::scope())
        .service(all::scope())
}
