/// generic error occuring during json data deserialization, serves as a json replacement of the default
/// actix_web one.
///
#[derive(Debug)]
pub struct JsonDeserializeError(pub actix_web::error::JsonPayloadError);

impl std::fmt::Display for JsonDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "invalid_json_data:{}", self.0.to_string())
    }
}

impl actix_web::error::ResponseError for JsonDeserializeError {
    fn error_response(self: &Self) -> actix_web::HttpResponse { 
        actix_web::HttpResponse::BadRequest()
            .json(serde_json::json!({ "code": self.to_string() }))
    }
}
