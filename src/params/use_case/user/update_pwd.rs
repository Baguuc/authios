pub struct UpdatePwdParams {
    pub token: String,
    pub encryption_key: String,
    pub new_pwd: String
}

pub struct UpdatePwdParamsBuilder {
    token: Option<String>,
    encryption_key: Option<String>,
    new_pwd: Option<String>
}

impl UpdatePwdParamsBuilder {
    pub fn new() -> Self {
        return Self {
            token: None,
            encryption_key: None,
            new_pwd: None
        };
    }
    
    pub fn set_token(self, token: String) -> Self {
        return Self {
            token: Some(token),
            ..self
        };
    }
    
    pub fn set_encryption_key(self, encryption_key: String) -> Self {
        return Self {
            encryption_key: Some(encryption_key),
            ..self
        };
    }
    
    pub fn set_new_pwd(self, new_pwd: String) -> Self {
        return Self {
            new_pwd: Some(new_pwd),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UpdatePwdParams> {
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }
        
        if self.new_pwd.is_none() {
            return None;
        }

        let params = UpdatePwdParams {
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap(),
            new_pwd: self.new_pwd.unwrap()
        };

        return Some(params);
    }
}
