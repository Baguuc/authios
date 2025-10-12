pub enum SpecificUserCheckServicePermissionResponse {
    Ok(bool),
    Unauthorized,
    UserNotFound,
    PermissionNotFound
}

impl From<Result<bool, crate::errors::use_case::SpecificUserCheckServicePermissionError>> for SpecificUserCheckServicePermissionResponse {
    fn from(result: Result<bool, crate::errors::use_case::SpecificUserCheckServicePermissionError>) -> Self {
        use crate::errors::use_case::SpecificUserCheckServicePermissionError as Error;

        match result {
            Ok(has_permission) => Self::Ok(has_permission),
            Err(error) => match error {
                Error::NotFound => Self::UserNotFound,
                Error::Unauthorized => Self::Unauthorized,
                Error::PermissionNotFound => Self::PermissionNotFound
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for SpecificUserCheckServicePermissionResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(has) => HttpResponse::Ok()
                .json(json!({ "has_permission": has })),
            
            Self::UserNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
            Self::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" })),
            
            Self::Unauthorized => HttpResponse::NotFound()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
