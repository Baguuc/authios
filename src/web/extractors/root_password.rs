/// extracts the root password from HttpRequest with the extractor pattern.
/// Example:
/// ```rust
/// #[get("/permissions/resource")]
/// async fn controller(
///     root_password: RootPasswordExtractor,
///     // ...
/// ) -> HttpResponse {
///     // this is how to access the value of extracted root password
///     let root_password = root_password.0; 
///     
///     // ...
/// }
/// ```
///
pub struct RootPasswordExtractor(pub String);

impl actix_web::FromRequest for RootPasswordExtractor {
    type Error = crate::errors::web::RootPasswordExtractionError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        use std::future::ready;
        use crate::errors::web::RootPasswordExtractionError;

        let raw_token = match req.headers().get("authorization") {
            Some(token) => token,
            None => return ready(Err(RootPasswordExtractionError::NotFound))
        };

        let token = match raw_token.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => return ready(Err(RootPasswordExtractionError::Invalid))
        };

        if !token.starts_with("Bearer ") {
            return ready(Err(RootPasswordExtractionError::WrongType))
        }
        
        let stripped_token = token
            .replace("Bearer ", "")
            .to_string();
        
        std::future::ready(Ok(Self(stripped_token)))
    }
}

