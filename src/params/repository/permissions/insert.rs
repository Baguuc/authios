/// # PermissionInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::permissions::PermissionsRepository::insert] method.
///
pub struct PermissionInsertParams {
    /// the name of the permission to insert
    ///
    pub name: String
}

pub struct InsertParamsBuilder {
    name: Option<String>
}

impl InsertParamsBuilder {
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
    
    pub fn build(self) -> Option<PermissionInsertParams> {
        if self.name.is_none() {
            return None;
        }

        let params = PermissionInsertParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
