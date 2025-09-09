pub struct RevokeParams {
    pub group_name: String,
    pub user_login: String
}

pub struct RevokeParamsBuilder {
    group_name: Option<String>,
    user_login: Option<String>
}

impl RevokeParamsBuilder {
    pub fn new() -> Self {
        return Self {
            group_name: None,
            user_login: None
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
    
    pub fn build(self) -> Option<RevokeParams> {
        if self.group_name.is_none() {
            return None;
        }
        
        if self.user_login.is_none() {
            return None;
        }

        let params = RevokeParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap()
        };

        return Some(params);
    }
}
