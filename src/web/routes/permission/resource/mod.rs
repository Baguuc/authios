pub mod create;
pub mod delete;
pub mod list_users;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/resource")
        .service(create::controller)
        .service(delete::controller)
        .service(list_users::controller)
}
