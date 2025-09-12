/// # UserRetrieveParams
///
/// Represent parameters for using the
/// [crate::repositories::users::UsersRepository::retrieve] method.
///
pub struct UserRetrieveParams {
    /// login of the user to retrieve
    ///
    pub login: String
}

pub struct RetrieveParamsBuilder {
    login: Option<String>
}

impl RetrieveParamsBuilder {
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
    
    pub fn build(self) -> Option<UserRetrieveParams> {
        if self.login.is_none() {
            return None;
        }

        let params = UserRetrieveParams {
            login: self.login.unwrap()
        };

        return Some(params);
    }
}
