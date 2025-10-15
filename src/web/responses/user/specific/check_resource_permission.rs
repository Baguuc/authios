pub enum SpecificUserCheckResourcePermissionResponse {
    Ok(bool),
    Unauthorized,
    UserNotFound,
    PermissionNotFound
}

impl From<Result<bool, crate::errors::use_case::SpecificUserCheckResourcePermissionError>> for SpecificUserCheckResourcePermissionResponse {
    fn from(result: Result<bool, crate::errors::use_case::SpecificUserCheckResourcePermissionError>) -> Self {
        use crate::errors::use_case::SpecificUserCheckResourcePermissionError as Error;

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

impl Into<actix_web::HttpResponse> for SpecificUserCheckResourcePermissionResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(has) => HttpResponse::Ok()
                .json(json!({ "code": "ok", "has_permission": has })),
            
            Self::UserNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
            Self::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" })),
            
            Self::Unauthorized => HttpResponse::NotFound()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
