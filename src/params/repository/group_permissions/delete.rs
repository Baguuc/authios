/// # GroupPermissionDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::group_permissions::GroupPermissionsRepository::delete] method.
///
pub struct GroupPermissionDeleteParams {
    /// the name of the permission to grant
    ///
    pub permission_name: String,
    /// the name of the group to grant the permission to
    ///
    pub group_name: String,
}

pub struct DeleteParamsBuilder {
    permission_name: Option<String>,
    group_name: Option<String>
}

impl DeleteParamsBuilder {
    pub fn new() -> Self {
        return Self {
            permission_name: None,
            group_name: None
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
    
    pub fn build(self) -> Option<GroupPermissionDeleteParams> {
        if self.permission_name.is_none() {
            return None;
        }

        let params = GroupPermissionDeleteParams {
            permission_name: self.permission_name.unwrap(),
            group_name: self.group_name.unwrap()
        };

        return Some(params);
    }
}
