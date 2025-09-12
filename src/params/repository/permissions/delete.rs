/// # PermissionDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::permissions::PermissionsRepository::delete] method.
///
pub struct PermissionDeleteParams {
    /// the name of the permission to delete
    ///
    pub name: String
}

pub struct DeleteParamsBuilder {
    name: Option<String>
}

impl DeleteParamsBuilder {
    pub fn new() -> Self {
        return Self {
            name: None
        };
    }
    
    pub fn set_name(self, name: String) -> Self {
        return Self {
            name: Some(name),
            ..self
        };
    }
    
    pub fn build(self) -> Option<PermissionDeleteParams> {
        if self.name.is_none() {
            return None;
        }

        let params = PermissionDeleteParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
