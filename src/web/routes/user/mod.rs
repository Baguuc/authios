mod me;
mod specific;
mod register;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(register::controller)
        /*.service(specific::grant_resource_permission::controller)
        .service(specific::revoke_resource_permission::controller)*/
        .service(me::scope())
        .service(specific::scope())
}
