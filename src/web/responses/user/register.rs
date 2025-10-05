pub enum UserRegisterResponse {
    Ok,
    AlreadyExists
}

impl From<Result<(), crate::errors::use_case::UserRegisterError>> for UserRegisterResponse {
    fn from(result: Result<(), crate::errors::use_case::UserRegisterError>) -> Self {
        use crate::errors::use_case::UserRegisterError as Error;

        match result {
            Ok(_) => Self::Ok,
            Err(error) => match error {
                Error::AlreadyExists => Self::AlreadyExists
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for UserRegisterResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok => HttpResponse::Created()
                .json(json!({ "code": "ok" })),
            
            Self::AlreadyExists => HttpResponse::Conflict()
                .json(json!({ "code": "already_exists" }))
        }
    }
}
