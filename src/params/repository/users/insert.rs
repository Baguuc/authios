/// # UserInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::users::UsersRepository::insert] method.
///
pub struct UserInsertParams {
    /// login of the user to insert
    ///
    pub login: String,
    /// hashed password of the user to insert
    ///
    pub pwd: String
}

pub struct InsertParamsBuilder {
    login: Option<String>,
    pwd: Option<String>
}

impl InsertParamsBuilder {
    pub fn new() -> Self {
        return Self {
            login: None,
            pwd: None
        };
    }
    
    pub fn set_login(self, login: String) -> Self {
        return Self {
            login: Some(login),
            ..self
        };
    }
    
    pub fn set_pwd(self, pwd: String) -> Self {
        return Self {
            pwd: Some(pwd),
            ..self
        };
    }
    
    pub fn build(self) -> Option<UserInsertParams> {
        if self.login.is_none() {
            return None;
        }
        
        if self.pwd.is_none() {
            return None;
        }

        let params = UserInsertParams {
            login: self.login.unwrap(),
            pwd: self.pwd.unwrap()
        };

        return Some(params);
    }
}
