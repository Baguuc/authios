pub enum UserUpdateResponse {
    Ok,
    InvalidToken
}

impl From<Result<crate::models::User, crate::errors::use_case::UserUpdateError>> for UserUpdateResponse {
    fn from(result: Result<crate::models::User, crate::errors::use_case::UserUpdateError>) -> Self {
        use crate::errors::use_case::UserUpdateError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::InvalidToken => Self::InvalidToken
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for UserUpdateResponse {
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
