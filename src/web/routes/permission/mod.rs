mod resource;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/permissions")
        .service(resource::scope())
}
