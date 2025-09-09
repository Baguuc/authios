pub mod login;
pub mod info;
pub mod authorize;
pub mod update_pwd;
pub mod grant_group;
pub mod revoke_group;
pub mod register;
pub mod list_permissions;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("")
        .service(
            actix_web::web::scope("/user")
                .service(login::controller)
        )
        .service(
            actix_web::web::scope("/user")
                .guard(actix_web::guard::fn_guard(|ctx| ctx.head().headers().get("authorization").is_some()))
                .service(info::controller)
                .service(authorize::controller)
                .service(update_pwd::controller)
                .service(grant_group::controller)
                .service(revoke_group::controller)
        )
        .service(
            actix_web::web::scope("/users")
                .service(register::controller)
        )
}
