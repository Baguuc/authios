mod grant_resource_permission;
mod revoke_resource_permission;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(grant_resource_permission::controller)
        .service(revoke_resource_permission::controller)
}
