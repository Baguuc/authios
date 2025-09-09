pub struct InsertParams {
    pub login: String,
    pub pwd: String
}

pub struct InsertParamsBuilder {
    login: Option<String>,
    pwd: Option<String>
}

impl InsertParamsBuilder {
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
    
    pub fn build(self) -> Option<InsertParams> {
        if self.login.is_none() {
            return None;
        }
        
        if self.pwd.is_none() {
            return None;
        }

        let params = InsertParams {
            login: self.login.unwrap(),
            pwd: self.pwd.unwrap()
        };

        return Some(params);
    }
}
