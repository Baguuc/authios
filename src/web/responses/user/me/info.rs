pub enum LoggedUserInfoResponse {
    Ok(OkData),
    InvalidToken
}

impl LoggedUserInfoResponse {
    pub fn partialize_ok(self: Self, save_id: bool, save_login: bool, save_password: bool) -> Self {
        match self {
            Self::Ok(data) => {
                let ok_data = OkData { 
                    user: UserData {
                        id: if save_id
                            { data.user.id } else { None },
                        login: if save_login
                            { data.user.login.clone() } else { None },
                        password_hash: if save_password
                            { data.user.password_hash.clone() } else { None }
                    }
                };

                Self::Ok(ok_data)
            },
            _ => self
        }
    }
}

impl From<Result<crate::models::User, crate::errors::use_case::LoggedUserInfoError>> for LoggedUserInfoResponse {
    fn from(result: Result<crate::models::User, crate::errors::use_case::LoggedUserInfoError>) -> Self {
        use crate::errors::use_case::LoggedUserInfoError as Error;

        match result {
            Ok(user) => {
                Self::Ok(OkData {
                    user: UserData {
                        id: Some(user.id),
                        login: Some(user.login),
                        password_hash: Some(user.password_hash)
                    }
                })
            },
            Err(error) => match error {
                Error::InvalidToken => Self::InvalidToken,
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for LoggedUserInfoResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(data) => HttpResponse::Ok()
                .json(json!({ "code": "ok", "user": data })),
            
            Self::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
        }
    }
}

#[derive(serde::Serialize)]
pub struct OkData {
    user: UserData
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct UserData {
    id: Option<i32>,
    login: Option<String>,
    password_hash: Option<String>
}
