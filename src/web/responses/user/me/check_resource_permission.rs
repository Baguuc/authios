pub enum LoggedUserCheckResourcePermissionResponse {
    Ok(bool),
    InvalidToken,
    PermissionNotFound
}

impl From<Result<bool, crate::errors::use_case::LoggedUserCheckResourcePermissionError>> for LoggedUserCheckResourcePermissionResponse {
    fn from(result: Result<bool, crate::errors::use_case::LoggedUserCheckResourcePermissionError>) -> Self {
        use crate::errors::use_case::LoggedUserCheckResourcePermissionError as Error;

        match result {
            Ok(has_permission) => Self::Ok(has_permission),
            Err(error) => match error {
                Error::InvalidToken => Self::InvalidToken,
                Error::PermissionNotFound => Self::PermissionNotFound
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for LoggedUserCheckResourcePermissionResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(has) => HttpResponse::Ok()
                .json(json!({ "has_permission": has })),
            
            Self::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
            
            Self::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" }))
        }
    }
}
