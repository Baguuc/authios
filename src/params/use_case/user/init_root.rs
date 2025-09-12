/// # UserInitRootParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::init_root] method.
///
pub struct UserInitRootParams {
    /// pwd of the root user
    ///
    pub pwd: String
}

pub struct UserInitRootParamsBuilder {
    pwd: Option<String>
}

impl UserInitRootParamsBuilder {
    pub fn new() -> Self {
        return Self {
            pwd: None
        };
    }
    
    pub fn set_pwd(self, pwd: String) -> Self {
        return Self {
            pwd: Some(pwd),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UserInitRootParams> {
        if self.pwd.is_none() {
            return None;
        }

        let params = UserInitRootParams {
            pwd: self.pwd.unwrap()
        };

        return Some(params);
    }
}
