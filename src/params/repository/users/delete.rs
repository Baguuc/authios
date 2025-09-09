pub struct DeleteParams {
    pub login: String
}

pub struct DeleteParamsBuilder {
    login: Option<String>
}

impl DeleteParamsBuilder {
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
    
    pub fn build(self) -> Option<DeleteParams> {
        if self.login.is_none() {
            return None;
        }

        let params = DeleteParams {
            login: self.login.unwrap()
        };

        return Some(params);
    }
}
