pub struct GrantParams {
    pub permission_name: String,
    pub group_name: String
}

pub struct GrantParamsBuilder {
    permission_name: Option<String>,
    group_name: Option<String>
}

impl GrantParamsBuilder {
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
    
    pub fn build(self) -> Option<GrantParams> {
        if self.permission_name.is_none() {
            return None;
        }
        
        if self.group_name.is_none() {
            return None;
        }

        let params = GrantParams {
            permission_name: self.permission_name.unwrap(),
            group_name: self.group_name.unwrap()
        };

        return Some(params);
    }
}
