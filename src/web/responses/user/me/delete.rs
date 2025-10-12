pub enum LoggedUserDeleteResponse {
    Ok,
    InvalidToken
}

impl From<Result<(), crate::errors::use_case::LoggedUserDeleteError>> for LoggedUserDeleteResponse {
    fn from(result: Result<(), crate::errors::use_case::LoggedUserDeleteError>) -> Self {
        use crate::errors::use_case::LoggedUserDeleteError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::InvalidToken => Self::InvalidToken
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for LoggedUserDeleteResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
            
            Self::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" }))
        }
    }
}
