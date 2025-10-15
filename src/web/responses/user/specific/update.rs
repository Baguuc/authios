pub enum SpecificUserUpdateResponse {
    Ok,
    NotFound,
    Unauthorized
}

impl From<Result<crate::models::User, crate::errors::use_case::SpecificUserUpdateError>> for SpecificUserUpdateResponse {
    fn from(result: Result<crate::models::User, crate::errors::use_case::SpecificUserUpdateError>) -> Self {
        use crate::errors::use_case::SpecificUserUpdateError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::NotFound => Self::NotFound,
                Error::Unauthorized => Self::Unauthorized
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for SpecificUserUpdateResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Ok()
                .json(json!({ "code": "ok" })),
            
            Self::NotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}
