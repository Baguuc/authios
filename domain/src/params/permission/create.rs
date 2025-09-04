pub struct CreateParams {
    pub name: String,
    pub auth: crate::AuthParams
}

pub struct CreateParamsBuilder {
    name: Option<String>,
    auth: Option<crate::AuthParams>
}

impl CreateParamsBuilder {
    pub fn new() -> Self {
        return Self {
            name: None,
            auth: None
        };
    }
    
    pub fn set_name(self, name: String) -> Self {
        return Self {
            name: Some(name),
            ..self
        };
    }
    
    pub fn set_auth(self, auth: crate::AuthParams) -> Self {
        return Self {
            auth: Some(auth),
            ..self
        };
    }
    
    pub fn build(self) -> Option<CreateParams> {
        if self.name.is_none() {
            return None;
        }

        if self.auth.is_none() {
            return None;
        }

        let params = CreateParams {
            name: self.name.unwrap(),
            auth: self.auth.unwrap()
        };

        return Some(params);
    }
}
