/// # UserRegisterParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::register] method.
///
pub struct UserRegisterParams {
    /// login of the user to register
    ///
    pub login: String,
    /// password of the user to register
    ///
    pub pwd: String
}

pub struct UserRegisterParamsBuilder {
    login: Option<String>,
    pwd: Option<String>
}

impl UserRegisterParamsBuilder {
    pub fn new() -> Self {
        return Self {
            login: None,
            pwd: None
        };
    }
    
    pub fn set_login(self, login: String) -> Self {
        return Self {
            login: Some(login),
            ..self
        };
    }
    
    pub fn set_pwd(self, pwd: String) -> Self {
        return Self {
            pwd: Some(pwd),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UserRegisterParams> {
        if self.login.is_none() {
            return None;
        }
        
        if self.pwd.is_none() {
            return None;
        }

        let params = UserRegisterParams {
            login: self.login.unwrap(),
            pwd: self.pwd.unwrap()
        };

        return Some(params);
    }
}
