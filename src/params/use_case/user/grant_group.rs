pub struct GrantParams {
    pub group_name: String,
    pub user_login: String,
    pub token: String,
    pub encryption_key: String
}

pub struct GrantParamsBuilder {
    group_name: Option<String>,
    user_login: Option<String>,
    token: Option<String>,
    encryption_key: Option<String>
}

impl GrantParamsBuilder {
    pub fn new() -> Self {
        return Self {
            group_name: None,
            user_login: None,
            token: None,
            encryption_key: None
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
    
    pub fn build(self) -> Option<GrantParams> {
        if self.group_name.is_none() {
            return None;
        }

        if self.user_login.is_none() {
            return None;
        }
        
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }

        let params = GrantParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap(),
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
