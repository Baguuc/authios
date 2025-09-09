pub struct ListPermissionsParams {
    pub token: String,
    pub encryption_key: String
}

pub struct ListPermissionsParamsBuilder {
    token: Option<String>,
    encryption_key: Option<String>
}

impl ListPermissionsParamsBuilder {
    pub fn new() -> Self {
        return Self {
            token: None,
            encryption_key: None
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
    
    pub fn build(self) -> Option<ListPermissionsParams> {
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }

        let params = ListPermissionsParams {
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
