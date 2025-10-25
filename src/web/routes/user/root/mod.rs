mod check_password;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/root")
        .service(check_password::controller)
}
