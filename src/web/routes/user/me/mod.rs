mod info;
mod login;
mod update;
mod delete;
mod list_resource_permissions;
mod check_resource_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/me")
        .service(info::controller)
        .service(login::controller)
        .service(update::controller)
        .service(delete::controller)
        .service(list_resource_permissions::controller)
        .service(check_resource_permission::controller)
}
