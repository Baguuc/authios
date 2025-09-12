/// # GroupInsertParams
///
/// Represent parameters for using the
/// [crate::repositories::groups::GroupsRepository::insert] method.
///
pub struct GroupInsertParams {
    /// the name of the group to insert
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
    
    pub fn build(self) -> Option<GroupInsertParams> {
        if self.name.is_none() {
            return None;
        }

        let params = GroupInsertParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
