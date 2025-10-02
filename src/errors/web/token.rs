/// Represents a error that happens during extracting token from HTTP request's headers
///
#[derive(Debug)]
pub enum TokenExtractionError {
    NotFound,
    Invalid,
    WrongType
}

impl std::fmt::Display for TokenExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "")
    }
}

impl actix_web::error::ResponseError for TokenExtractionError {
    fn error_response(self: &Self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse; 

        match self {
            Self::NotFound => HttpResponse::BadRequest()
                .body("JWT token missing in the \"authorization\" header."),
            Self::Invalid => HttpResponse::BadRequest()
                .body("JWT token in the \"authorization\" header is of wrong string format (propably containing invalid characters)."),
            Self::WrongType => HttpResponse::BadRequest()
                .body("JWT token in the \"authorization\" header is of wrong type (not \"Bearer\")."),
        }
    }
}
