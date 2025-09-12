/// # GroupGrantPermissionParams
///
/// Represent parameters for using the
/// [crate::use_cases::group::GroupsUseCase::grant_permission] method.
///
pub struct GroupGrantPermissionParams {
    /// name of the permission to grant
    ///
    pub permission_name: String,
    /// name of the group to grant the permission to
    ///
    pub group_name: String,
    /// token of the user that want to perform the operation
    ///
    pub token: String,
    /// system's JWT encryption key
    ///
    pub encryption_key: String
}

pub struct GroupGrantPermissionParamsBuilder {
    permission_name: Option<String>,
    group_name: Option<String>,
    token: Option<String>,
    encryption_key: Option<String>
}

impl GroupGrantPermissionParamsBuilder {
    pub fn new() -> Self {
        return Self {
            permission_name: None,
            group_name: None,
            token: None,
            encryption_key: None
        };
    }
    
    pub fn set_permission_name(self, permission_name: String) -> Self {
        return Self {
            permission_name: Some(permission_name),
            ..self
        };
    }
    
    pub fn set_group_name(self, group_name: String) -> Self {
        return Self {
            group_name: Some(group_name),
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
    
    pub fn build(self) -> Option<GroupGrantPermissionParams> {
        if self.permission_name.is_none() {
            return None;
        }

        if self.group_name.is_none() {
            return None;
        }
        
        if self.token.is_none() {
            return None;
        }
        
        if self.encryption_key.is_none() {
            return None;
        }

        let params = GroupGrantPermissionParams {
            permission_name: self.permission_name.unwrap(),
            group_name: self.group_name.unwrap(),
            token: self.token.unwrap(),
            encryption_key: self.encryption_key.unwrap()
        };

        return Some(params);
    }
}
