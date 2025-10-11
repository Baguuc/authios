pub enum UserDeleteAsAdminResponse {
    Ok,
    Unauthorized,
    NotFound
}

impl From<Result<(), crate::errors::use_case::AdminDeleteUserError>> for UserDeleteAsAdminResponse {
    fn from(result: Result<(), crate::errors::use_case::AdminDeleteUserError>) -> Self {
        use crate::errors::use_case::AdminDeleteUserError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::Unauthorized => Self::Unauthorized,
                Error::NotFound => Self::NotFound
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for UserDeleteAsAdminResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
            
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "unauthorized" })),
            
            Self::NotFound => HttpResponse::NotFound()
                .json(json!({ "code": "not_found" }))
        }
    }
}
