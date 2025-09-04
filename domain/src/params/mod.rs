pub mod group;
pub use group::*;

pub mod permission;
pub use permission::*;

pub struct AuthParams {
    pub token: String,
    pub encoding_key: String
}

pub struct AuthParamsBuilder {
    token: Option<String>,
    encoding_key: Option<String>
}

impl AuthParamsBuilder {
    pub fn new() -> Self {
        return Self {
            token: None,
            encoding_key: None
        };
    }

    pub fn set_token(self, token: String) -> Self {
        return Self {
            token: Some(token),
            ..self
        };
    }
    
    pub fn set_encoding_key(self, encoding_key: String) -> Self {
        return Self {
            encoding_key: Some(encoding_key),
            ..self
        };
    }

    pub fn build(self) -> Option<AuthParams> {
        if self.token.is_none() {
            return None;
        }

        if self.encoding_key.is_none() {
            return None;
        }

        let params = AuthParams {
            token: self.token.unwrap(),
            encoding_key: self.encoding_key.unwrap()
        };

        return Some(params);
    }
}
