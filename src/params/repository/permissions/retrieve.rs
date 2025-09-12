/// # PermissionRetrieveParams
///
/// Represent parameters for using the
/// [crate::repositories::permissions::PermissionsRepository::retrieve] method.
///
pub struct PermissionRetrieveParams {
    /// the name of the permission to retrieve
    ///
    pub name: String
}

pub struct RetrieveParamsBuilder {
    name: Option<String>
}

impl RetrieveParamsBuilder {
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
    
    pub fn build(self) -> Option<PermissionRetrieveParams> {
        if self.name.is_none() {
            return None;
        }

        let params = PermissionRetrieveParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
