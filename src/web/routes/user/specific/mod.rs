pub mod list_resource_permissions;
pub mod update;
pub mod delete;
pub mod grant_resource_permission;
pub mod revoke_resource_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/{user_id}")
        .service(delete::controller)
        .service(grant_resource_permission::controller)
        .service(revoke_resource_permission::controller)
}
