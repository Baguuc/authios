pub mod create;
pub mod delete;
pub mod grant;
pub mod revoke;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/groups")
        .guard(actix_web::guard::fn_guard(|ctx| ctx.head().headers().get("authorization").is_some()))
        .service(create::controller)
        .service(delete::controller)
        .service(grant::controller)
        .service(revoke::controller)
}
