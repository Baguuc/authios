pub struct DeleteParams {
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
    
    pub fn build(self) -> Option<DeleteParams> {
        if self.name.is_none() {
            return None;
        }

        let params = DeleteParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
