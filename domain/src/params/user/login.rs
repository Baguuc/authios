pub struct LoginParams {
    pub login: String,
    pub pwd: String,
    pub encryption_key: String
}

pub struct LoginParamsBuilder {
    login: Option<String>,
    pwd: Option<String>,
    encryption_key: Option<String>
}

impl LoginParamsBuilder {
    pub fn new() -> Self {
        return Self {
            login: None,
            pwd: None,
            encryption_key: None
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
    
    pub fn set_encryption_key(self, encryption_key: String) -> Self {
        return Self {
            encryption_key: Some(encryption_key),
            ..self
        };
    }
    
    pub fn build(self) -> Option<LoginParams> {
        if self.login.is_none() {
            return None;
        }
        
        if self.pwd.is_none() {
            return None;
        }

        if self.encryption_key.is_none() {
            return None;
        }

        let params = LoginParams {
            login: self.login.unwrap(),
            pwd: self.pwd.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
