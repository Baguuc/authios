pub struct GrantParams {
    pub name: String,
    pub group_name: String,
    pub auth: crate::AuthParams
}

pub struct GrantParamsBuilder {
    name: Option<String>,
    group_name: Option<String>,
    auth: Option<crate::AuthParams>
}

impl GrantParamsBuilder {
    pub fn new() -> Self {
        return Self {
            name: None,
            group_name: None,
            auth: None
        };
    }
    
    pub fn set_name(self, name: String) -> Self {
        return Self {
            name: Some(name),
            ..self
        };
    }
    
    pub fn set_group_name(self, group_name: String) -> Self {
        return Self {
            group_name: Some(group_name),
            ..self
        };
    }
    
    pub fn set_auth(self, auth: crate::AuthParams) -> Self {
        return Self {
            auth: Some(auth),
            ..self
        };
    }
    
    pub fn build(self) -> Option<GrantParams> {
        if self.name.is_none() {
            return None;
        }
        
        if self.group_name.is_none() {
            return None;
        }

        if self.auth.is_none() {
            return None;
        }

        let params = GrantParams {
            name: self.name.unwrap(),
            group_name: self.group_name.unwrap(),
            auth: self.auth.unwrap()
        };

        return Some(params);
    }
}
