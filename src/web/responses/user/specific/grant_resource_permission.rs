pub enum SpecificUserGrantResourcePermissionResponse {
    Ok,
    UserNotFound,
    PermissionNotFound,
    AlreadyAdded,
    Unauthorized
}

impl From<Result<(), crate::errors::use_case::SpecificUserGrantResourcePermissionError>> for SpecificUserGrantResourcePermissionResponse {
    fn from(result: Result<(), crate::errors::use_case::SpecificUserGrantResourcePermissionError>) -> Self {
        use crate::errors::use_case::SpecificUserGrantResourcePermissionError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::UserNotFound => Self::UserNotFound,
                Error::PermissionNotFound => Self::PermissionNotFound,
                Error::AlreadyAdded => Self::AlreadyAdded,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for SpecificUserGrantResourcePermissionResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
             
            Self::UserNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
            Self::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" })),
            
            Self::AlreadyAdded => HttpResponse::Conflict()
                .json(json!({ "code": "already_added" })),
            
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
