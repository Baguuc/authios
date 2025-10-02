pub struct TokenExtractor(pub String);

impl actix_web::FromRequest for TokenExtractor {
    type Error = crate::errors::web::TokenExtractionError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        use std::future::ready;
        use crate::errors::web::TokenExtractionError;

        let raw_token = match req.headers().get("authorization") {
            Some(token) => token,
            None => return ready(Err(TokenExtractionError::NotFound))
        };

        let token = match raw_token.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return ready(Err(TokenExtractionError::Invalid))
        };
        
        if !token.starts_with("Bearer ") {
            return ready(Err(TokenExtractionError::WrongType))
        }
        
        let stripped_token = token
            .replace("Bearer ", "")
            .to_string();
        
        std::future::ready(Ok(Self(stripped_token)))
    }
}

