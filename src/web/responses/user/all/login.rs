pub enum AllUserLoginResponse {
    Ok(String),
    NotFound,
    WrongPassword
}

impl From<Result<String, crate::errors::use_case::AllUserLoginError>> for AllUserLoginResponse {
    fn from(result: Result<String, crate::errors::use_case::AllUserLoginError>) -> Self {
        use crate::errors::use_case::AllUserLoginError as Error;

        match result {
            Ok(token) => Self::Ok(token),
            Err(error) => match error {
                Error::NotFound => Self::NotFound,
                Error::WrongPassword => Self::WrongPassword
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for AllUserLoginResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(token) => HttpResponse::Ok()
                .json(json!({ "code": "ok", "token": token })),
            
            Self::NotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),

            Self::WrongPassword => HttpResponse::Unauthorized()
                .json(json!({ "code": "wrong_password" })),
        }
    }
}
