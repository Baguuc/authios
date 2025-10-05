pub enum UserLoginResponse {
    Ok(String),
    NotFound,
    WrongPassword
}

impl From<Result<String, crate::errors::use_case::UserLoginError>> for UserLoginResponse {
    fn from(result: Result<String, crate::errors::use_case::UserLoginError>) -> Self {
        use crate::errors::use_case::UserLoginError as Error;

        match result {
            Ok(token) => Self::Ok(token),
            Err(error) => match error {
                Error::NotFound => Self::NotFound,
                Error::WrongPassword => Self::WrongPassword
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for UserLoginResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(token) => HttpResponse::Ok()
                .json(json!({ "token": token })),
            
            Self::NotFound => HttpResponse::NotFound()
                .json(json!({ "code": "not_found" })),

            Self::WrongPassword => HttpResponse::Unauthorized()
                .json(json!({ "code": "wrong_password" })),
        }
    }
}
