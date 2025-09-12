/// # UserGroupDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::user_groups::UserGroupsRepository::delete] method.
///
pub struct UserGroupDeleteParams {
    /// the name of the group to revoke
    ///
    pub group_name: String,
    /// the name of the user to revoke the group from
    ///
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
    
    pub fn build(self) -> Option<UserGroupDeleteParams> {
        if self.group_name.is_none() {
            return None;
        }

        let params = UserGroupDeleteParams {
            group_name: self.group_name.unwrap(),
            user_login: self.user_login.unwrap()
        };

        return Some(params);
    }
}
