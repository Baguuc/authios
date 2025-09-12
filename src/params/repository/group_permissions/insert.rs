/// # GroupPermissionInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::group_permissions::GroupPermissionsRepository::insert] method.
///
pub struct GroupPermissionInsertParams {
    /// the name of the permission to revoke
    ///
    pub permission_name: String,
    /// the name of the group to revoke the permission from
    ///
    pub group_name: String,
}

pub struct InsertParamsBuilder {
    permission_name: Option<String>,
    group_name: Option<String>
}

impl InsertParamsBuilder {
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
    
    pub fn build(self) -> Option<GroupPermissionInsertParams> {
        if self.permission_name.is_none() {
            return None;
        }

        let params = GroupPermissionInsertParams {
            permission_name: self.permission_name.unwrap(),
            group_name: self.group_name.unwrap()
        };

        return Some(params);
    }
}
