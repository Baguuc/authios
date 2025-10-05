pub enum ResourcePermissionDeleteResponse {
    Ok,
    NotFound,
    Unauthorized
}

impl From<Result<(), crate::errors::use_case::ResourcePermissionDeleteError>> for ResourcePermissionDeleteResponse {
    fn from(result: Result<(), crate::errors::use_case::ResourcePermissionDeleteError>) -> Self {
        use crate::errors::use_case::ResourcePermissionDeleteError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::NotFound => Self::NotFound,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for ResourcePermissionDeleteResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
            
            Self::NotFound => HttpResponse::Conflict()
                .json(json!({ "code": "not_found" })),

            Self::Unauthorized => HttpResponse::Conflict()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
