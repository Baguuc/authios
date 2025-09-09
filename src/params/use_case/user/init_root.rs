pub struct InitRootParams {
    pub pwd: String
}

pub struct InitRootParamsBuilder {
    pwd: Option<String>
}

impl InitRootParamsBuilder {
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
    
    pub fn build(self) -> Option<InitRootParams> {
        if self.pwd.is_none() {
            return None;
        }

        let params = InitRootParams {
            pwd: self.pwd.unwrap()
        };

        return Some(params);
    }
}
