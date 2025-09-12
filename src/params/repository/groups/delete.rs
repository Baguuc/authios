/// # GroupDeleteParams
///
/// Represent parameters for using the
/// [crate::repositories::groups::GroupsRepository::delete] method.
///
pub struct GroupDeleteParams {
    /// the name of the group to delete
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
    
    pub fn build(self) -> Option<GroupDeleteParams> {
        if self.name.is_none() {
            return None;
        }

        let params = GroupDeleteParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
