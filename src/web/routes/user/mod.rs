mod me;
mod specific;
mod all;
mod root;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(me::scope())
        .service(root::scope())
        .service(specific::scope())
        .service(all::scope())
}
