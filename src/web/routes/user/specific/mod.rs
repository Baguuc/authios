pub mod update;
pub mod delete;
pub mod list_resource_permissions;
pub mod check_resource_permission;
pub mod grant_resource_permission;
pub mod revoke_resource_permission;
pub mod check_service_permission;
pub mod grant_service_permission;
pub mod revoke_service_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/{user_id}")
        .service(delete::controller)
        .service(update::controller)
        .service(list_resource_permissions::controller)
        .service(check_resource_permission::controller)
        .service(grant_resource_permission::controller)
        .service(revoke_resource_permission::controller)
        .service(check_service_permission::controller)
        .service(grant_service_permission::controller)
        .service(revoke_service_permission::controller)
}
