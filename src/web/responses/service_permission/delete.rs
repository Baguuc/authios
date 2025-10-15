pub enum ServicePermissionDeleteResponse {
    Ok,
    NotFound,
    Unauthorized
}

impl From<Result<(), crate::errors::use_case::ServicePermissionDeleteError>> for ServicePermissionDeleteResponse {
    fn from(result: Result<(), crate::errors::use_case::ServicePermissionDeleteError>) -> Self {
        use crate::errors::use_case::ServicePermissionDeleteError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::NotFound => Self::NotFound,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for ServicePermissionDeleteResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
            
            Self::NotFound => HttpResponse::Conflict()
                .json(json!({ "code": "permission_not_found" })),

            Self::Unauthorized => HttpResponse::Conflict()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
