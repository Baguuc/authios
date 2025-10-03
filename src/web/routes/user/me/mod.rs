mod retrieve_from_token;
mod login;
mod update;
mod list_resource_permissions;
mod check_resource_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/me")
        .service(retrieve_from_token::controller)
        .service(login::controller)
        .service(update::controller)
        .service(list_resource_permissions::controller)
        .service(check_resource_permission::controller)
}
