/// generic error occuring during path data deserialization, serves as a json replacement of the default
/// actix_web one.
///
#[derive(Debug)]
pub struct PathDeserializeError(pub actix_web::error::PathError);

impl std::fmt::Display for PathDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "invalid_path_data:{}", self.0.to_string())
    }
}

impl actix_web::error::ResponseError for PathDeserializeError {
    fn error_response(self: &Self) -> actix_web::HttpResponse { 
        actix_web::HttpResponse::BadRequest()
            .json(serde_json::json!({ "code": self.to_string() }))
    }
}
