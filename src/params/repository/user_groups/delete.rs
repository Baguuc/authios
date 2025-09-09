pub struct DeleteParams {
    pub group_name: String,
    pub user_login: String,
}

pub struct DeleteParamsBuilder {
    group_name: Option<String>,
    user_login: Option<String>
}

impl DeleteParamsBuilder {
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
    
    pub fn build(self) -> Option<DeleteParams> {
        if self.group_name.is_none() {
            return None;
        }

        let params = DeleteParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap()
        };

        return Some(params);
    }
}
