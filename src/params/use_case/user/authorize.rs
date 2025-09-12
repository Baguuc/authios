/// # UserAuthorizeParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::authorize] method.
///
pub struct UserAuthorizeParams {
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String,
    /// name of the permission to check for
    ///
    pub permission_name: String
}

pub struct UserAuthorizeParamsBuilder {
    token: Option<String>,
    encryption_key: Option<String>,
    permission_name: Option<String>
}

impl UserAuthorizeParamsBuilder {
    pub fn new() -> Self {
        return Self {
            token: None,
            encryption_key: None,
            permission_name: None
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
    
    pub fn set_permission_name(self, permission_name: String) -> Self {
        return Self {
            permission_name: Some(permission_name),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UserAuthorizeParams> {
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }
        
        if self.permission_name.is_none() {
            return None;
        }

        let params = UserAuthorizeParams {
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap(),
            permission_name: self.permission_name.unwrap()
        };

        return Some(params);
    }
}
