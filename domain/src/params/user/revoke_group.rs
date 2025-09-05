pub struct RevokeParams {
    pub group_name: String,
    pub user_login: String,
    pub auth: crate::AuthParams
}

pub struct RevokeParamsBuilder {
    group_name: Option<String>,
    user_login: Option<String>,
    auth: Option<crate::AuthParams>
}

impl RevokeParamsBuilder {
    pub fn new() -> Self {
        return Self {
            group_name: None,
            user_login: None,
            auth: None
        };
    }
    
    pub fn set_group_name(self, group_name: String) -> Self {
        return Self {
            group_name: Some(group_name),
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
        if self.group_name.is_none() {
            return None;
        }
        
        if self.user_login.is_none() {
            return None;
        }

        if self.auth.is_none() {
            return None;
        }

        let params = RevokeParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap(),
            auth: self.auth.unwrap()
        };

        return Some(params);
    }
}
