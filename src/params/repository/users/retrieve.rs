pub struct RetrieveParams {
    pub login: String
}

pub struct RetrieveParamsBuilder {
    login: Option<String>
}

impl RetrieveParamsBuilder {
    pub fn new() -> Self {
        return Self {
            login: None
        };
    }
    
    pub fn set_login(self, login: String) -> Self {
        return Self {
            login: Some(login),
            ..self
        };
    }
    
    pub fn build(self) -> Option<RetrieveParams> {
        if self.login.is_none() {
            return None;
        }

        let params = RetrieveParams {
            login: self.login.unwrap()
        };

        return Some(params);
    }
}
