pub struct DeleteParams {
    pub name: String,
    pub token: String,
    pub encryption_key: String
}

pub struct DeleteParamsBuilder {
    name: Option<String>,
    token: Option<String>,
    encryption_key: Option<String>
}

impl DeleteParamsBuilder {
    pub fn new() -> Self {
        return Self {
            name: None,
            token: None,
            encryption_key: None
        };
    }
    
    pub fn set_name(self, name: String) -> Self {
        return Self {
            name: Some(name),
            ..self
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
    
    pub fn build(self) -> Option<DeleteParams> {
        if self.name.is_none() {
            return None;
        }
        
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }

        let params = DeleteParams {
            name: self.name.unwrap(),
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
