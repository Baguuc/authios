#[actix_web::get("/password")]
pub async fn controller(
    password: crate::web::extractors::RootPasswordExtractor,
    config: actix_web::web::Data<crate::config::Config>
) -> actix_web::HttpResponse {
    use crate::params::use_case::RootUserCheckPasswordParams as Params;
    use crate::use_cases::RootUserUseCase as UseCase;
    use crate::web::responses::RootUserCheckPasswordResponse as Response;

    let params = Params {
        password: &password.0,
        root_password: &config.root.password
    };
    
    let response: Response = UseCase::check_password(params).into();
    
    response.into()
}
