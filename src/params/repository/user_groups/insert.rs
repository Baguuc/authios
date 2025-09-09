pub struct InsertParams {
    pub user_login: String,
    pub group_name: String,
}

pub struct InsertParamsBuilder {
    user_login: Option<String>,
    group_name: Option<String>
}

impl InsertParamsBuilder {
    pub fn new() -> Self {
        return Self {
            user_login: None,
            group_name: None
        };
    }
    
    pub fn set_user_login(self, user_login: String) -> Self {
        return Self {
            user_login: Some(user_login),
            ..self
        };
    }
    
    pub fn set_group_name(self, group_name: String) -> Self {
        return Self {
            group_name: Some(group_name),
            ..self
        };
    }
    
    pub fn build(self) -> Option<InsertParams> {
        if self.user_login.is_none() {
            return None;
        }

        let params = InsertParams {
            user_login: self.user_login.unwrap(),
            group_name: self.group_name.unwrap()
        };

        return Some(params);
    }
}
