pub struct RetrieveParams {
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
    
    pub fn build(self) -> Option<RetrieveParams> {
        if self.name.is_none() {
            return None;
        }

        let params = RetrieveParams {
            name: self.name.unwrap()
        };

        return Some(params);
    }
}
