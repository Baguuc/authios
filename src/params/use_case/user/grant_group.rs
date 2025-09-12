/// # UserGrantGroupParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::grant_group] method.
///
pub struct UserGrantGroupParams {
    /// name of the group to grant
    ///
    pub group_name: String,
    /// login of the user to grant the group to
    ///
    pub user_login: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}

pub struct UserGrantGroupParamsBuilder {
    group_name: Option<String>,
    user_login: Option<String>,
    token: Option<String>,
    encryption_key: Option<String>
}

impl UserGrantGroupParamsBuilder {
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
    
    pub fn build(self) -> Option<UserGrantGroupParams> {
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

        let params = UserGrantGroupParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap(),
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
