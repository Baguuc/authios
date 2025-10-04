/// Represents a error that happens during extracting root password from HTTP request's headers
///
#[derive(thiserror::Error, Debug)]
pub enum RootPasswordExtractionError {
    #[error("authorization_not_found")] 
    NotFound,
    #[error("invalid_authorization")]
    Invalid,
    #[error("wrong_authorization_type")]
    WrongType
}

impl actix_web::error::ResponseError for RootPasswordExtractionError {
    fn error_response(self: &Self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest()
            .json(serde_json::json!({ "code": self.to_string() }))
    }
}
