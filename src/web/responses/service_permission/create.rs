pub enum ServicePermissionCreateResponse {
    Ok,
    AlreadyExists,
    Unauthorized
}

impl From<Result<(), crate::errors::use_case::ServicePermissionCreateError>> for ServicePermissionCreateResponse {
    fn from(result: Result<(), crate::errors::use_case::ServicePermissionCreateError>) -> Self {
        use crate::errors::use_case::ServicePermissionCreateError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::AlreadyExists => Self::AlreadyExists,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for ServicePermissionCreateResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Created()
                .json(json!({ "code": "ok" })),
            
            Self::AlreadyExists => HttpResponse::Conflict()
                .json(json!({ "code": "already_exists" })),

            Self::Unauthorized => HttpResponse::Conflict()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
