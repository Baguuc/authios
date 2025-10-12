#[actix_web::delete("")]
pub async fn controller(
    path: actix_web::web::Path<Path>,
    password: crate::web::extractors::RootPasswordExtractor,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use crate::params::use_case::SpecificUserDeleteParams as Params;
    use crate::use_cases::SpecificUserUseCase as UseCase;
    use crate::web::responses::SpecificUserDeleteResponse as Response;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        id: &path.user_id,
        password: &password.0,
        root_password: &config.root.password 
    };

    let response: Response = UseCase::delete(params, &mut *database_client)
        .await
        .into();
    
    response.into()
}

#[derive(serde::Deserialize)]
struct Path {
    user_id: i32
}
