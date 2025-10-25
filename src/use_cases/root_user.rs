pub struct RootUserUseCase;

impl RootUserUseCase {
    pub fn check_password(params: crate::params::use_case::RootUserCheckPasswordParams) -> bool {
        params.password == params.root_password
    } 
}
