mod info;
mod update;
mod delete;
mod list_resource_permissions;
mod check_resource_permission;
mod check_service_permission;
mod login;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/me")
        .service(info::controller)
        .service(update::controller)
        .service(delete::controller)
        .service(list_resource_permissions::controller)
        .service(check_resource_permission::controller)
        .service(check_service_permission::controller)
        .service(login::controller)
}
