pub struct RevokeParams {
    pub name: String,
    pub user_login: String,
    pub auth: crate::AuthParams
}

pub struct RevokeParamsBuilder {
    name: Option<String>,
    user_login: Option<String>,
    auth: Option<crate::AuthParams>
}

impl RevokeParamsBuilder {
    pub fn new() -> Self {
        return Self {
            name: None,
            user_login: None,
            auth: None
        };
    }
    
    pub fn set_name(self, name: String) -> Self {
        return Self {
            name: Some(name),
            ..self
        };
    }
    
    pub fn set_user_login(self, user_login: String) -> Self {
        return Self {
            user_login: Some(user_login),
            ..self
        };
    }
    
    pub fn set_auth(self, auth: crate::AuthParams) -> Self {
        return Self {
            auth: Some(auth),
            ..self
        };
    }
    
    pub fn build(self) -> Option<RevokeParams> {
        if self.name.is_none() {
            return None;
        }
        
        if self.user_login.is_none() {
            return None;
        }

        if self.auth.is_none() {
            return None;
        }

        let params = RevokeParams {
            name: self.name.unwrap(),
            user_login: self.user_login.unwrap(),
            auth: self.auth.unwrap()
        };

        return Some(params);
    }
}
