/// generic error occuring during query data deserialization, serves as a json replacement of the default
/// actix_web one.
///
#[derive(Debug)]
pub struct QueryDeserializeError;

impl std::fmt::Display for QueryDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "invalid_query_data")
    }
}

impl actix_web::error::ResponseError for QueryDeserializeError {
    fn error_response(self: &Self) -> actix_web::HttpResponse { 
        actix_web::HttpResponse::BadRequest()
            .json(serde_json::json!({ "code": self.to_string() }))
    }
}
