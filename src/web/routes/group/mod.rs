pub mod create;
pub mod delete;
pub mod grant_permission;
pub mod revoke_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/groups")
        .guard(actix_web::guard::fn_guard(|ctx| ctx.head().headers().get("authorization").is_some()))
        .service(create::controller)
        .service(delete::controller)
        .service(grant_permission::controller)
        .service(revoke_permission::controller)
}
