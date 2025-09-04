pub mod login;
pub mod info;
pub mod authorize;
pub mod update_pwd;
pub mod grant;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/user")
        .service(login::controller)
        .service(info::controller)
        .service(authorize::controller)
        .service(update_pwd::controller)
        .service(grant::controller)
}
