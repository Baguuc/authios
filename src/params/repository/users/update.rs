pub struct UpdateParams {
    pub user_login: String,
    pub new_pwd: String
}

pub struct UpdateParamsBuilder {
    user_login: Option<String>,
    new_pwd: Option<String>
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        return Self {
            user_login: None,
            new_pwd: None
        };
    }
    
    pub fn set_user_login(self, user_login: String) -> Self {
        return Self {
            user_login: Some(user_login),
            ..self
        };
    }
    
    pub fn set_new_pwd(self, new_pwd: String) -> Self {
        return Self {
            new_pwd: Some(new_pwd),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UpdateParams> {
        if self.user_login.is_none() {
            return None;
        }
        
        if self.new_pwd.is_none() {
            return None;
        }

        let params = UpdateParams {
            user_login: self.user_login.unwrap(),
            new_pwd: self.new_pwd.unwrap()
        };

        return Some(params);
    }
}
