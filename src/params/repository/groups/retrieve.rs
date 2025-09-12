/// # GroupRetrieveParams
///
/// Represent parameters for using the
/// [crate::repositories::groups::GroupsRepository::retrieve] method.
///
pub struct GroupRetrieveParams {
    /// the name of the group to retrieve
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
    
    pub fn build(self) -> Option<GroupRetrieveParams> {
        if self.name.is_none() {
            return None;
        }

        let params = GroupRetrieveParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
