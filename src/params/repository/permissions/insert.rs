pub struct InsertParams {
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
    
    pub fn build(self) -> Option<InsertParams> {
        if self.name.is_none() {
            return None;
        }

        let params = InsertParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
