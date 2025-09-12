/// # UserListPermissionsParams
///
/// Represent parameters for using the
/// [crate::use_cases::user::UsersUseCase::list_permissions] method.
///
pub struct UserListPermissionsParams {
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}

pub struct UserListPermissionsParamsBuilder {
    token: Option<String>,
    encryption_key: Option<String>
}

impl UserListPermissionsParamsBuilder {
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
    
    pub fn build(self) -> Option<UserListPermissionsParams> {
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }

        let params = UserListPermissionsParams {
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
