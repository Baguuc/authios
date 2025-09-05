pub struct CreateParams {
    pub name: String
}

pub struct CreateParamsBuilder {
    name: Option<String>
}

impl CreateParamsBuilder {
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
    
    pub fn build(self) -> Option<CreateParams> {
        if self.name.is_none() {
            return None;
        }

        let params = CreateParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
