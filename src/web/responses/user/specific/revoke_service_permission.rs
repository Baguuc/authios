pub enum SpecificUserRevokeServicePermissionResponse {
    Ok,
    UserNotFound,
    PermissionNotFound,
    NotAddedYet,
    Unauthorized
}

impl From<Result<(), crate::errors::use_case::SpecificUserRevokeServicePermissionError>> for SpecificUserRevokeServicePermissionResponse {
    fn from(result: Result<(), crate::errors::use_case::SpecificUserRevokeServicePermissionError>) -> Self {
        use crate::errors::use_case::SpecificUserRevokeServicePermissionError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::UserNotFound => Self::UserNotFound,
                Error::PermissionNotFound => Self::PermissionNotFound,
                Error::NotAddedYet => Self::NotAddedYet,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for SpecificUserRevokeServicePermissionResponse {
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
            
            Self::NotAddedYet => HttpResponse::Conflict()
                .json(json!({ "code": "not_added_yet" })),
            
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
