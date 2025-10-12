mod register;
mod login;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(register::controller)
        .service(login::controller)
}
