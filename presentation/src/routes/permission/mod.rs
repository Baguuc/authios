pub mod create;
pub mod delete;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/permissions")
        .guard(actix_web::guard::fn_guard(|ctx| ctx.head().headers().get("authorization").is_some()))
        .service(create::controller)
        .service(delete::controller)
}
