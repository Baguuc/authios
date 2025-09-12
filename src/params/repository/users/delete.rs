/// # UserDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::users::UsersRepository::delete] method.
///
pub struct UserDeleteParams {
    /// login of the user to delete
    ///
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
    
    pub fn build(self) -> Option<UserDeleteParams> {
        if self.login.is_none() {
            return None;
        }

        let params = UserDeleteParams {
            login: self.login.unwrap()
        };

        return Some(params);
    }
}
