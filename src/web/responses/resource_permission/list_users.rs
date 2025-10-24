pub enum ResourcePermissionListUsersResponse {
    Ok(OkData),
    PermissionNotFound,
    Unauthorized
}

impl ResourcePermissionListUsersResponse { pub fn partialize_ok(
        self: Self,
        save_page_number: bool,
        save_total_page_count: bool,
        save_id: bool,
        save_login: bool,
        save_password_hash: bool
    ) -> Self {
        match self {
            Self::Ok(data) => {
                // there is 5 entries max so we don't worry about performance and time of loop
                // execution
                let users = data.users.iter()
                    .map(|e| {
                        UserData {         
                            id: if save_id 
                                { e.id.clone() } else { None },
                            login: if save_login 
                                { e.login.clone() } else { None },
                            password_hash: if save_password_hash
                                { e.password_hash.clone() } else { None }
                        }
                    })
                    .collect::<Vec<UserData>>();

                let ok_data = OkData { 
                    page_number: if save_page_number
                        { data.page_number } else { None },
                    total_page_count: if save_total_page_count
                        { data.total_page_count } else { None },

                    users
                };

                Self::Ok(ok_data)
            },
            _ => self
        }
    }
}

impl From<Result<crate::models::UsersPage, crate::errors::use_case::ResourcePermissionListUsersError>> for ResourcePermissionListUsersResponse {
    fn from(result: Result<crate::models::UsersPage, crate::errors::use_case::ResourcePermissionListUsersError>) -> Self {
        use crate::errors::use_case::ResourcePermissionListUsersError as Error;

        match result {
            Ok(page) => {
                Self::Ok(OkData {
                    page_number: Some(page.page_number),
                    total_page_count: Some(page.total_page_count),
                    users: page.users.iter().map(|e| e.into()).collect()
                })
            },
            Err(error) => match error {
                Error::PermissionNotFound => Self::PermissionNotFound,
                Error::Unauthorized => Self::Unauthorized,
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for ResourcePermissionListUsersResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(data) => HttpResponse::Ok()
                .json(json!({ "code": "ok", "list": data })),
            
            Self::PermissionNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "permission_not_found" })),
            
            Self::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "code": "unauthorized" }))
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct OkData {
    page_number: Option<u32>,
    total_page_count: Option<u32>,
    users: Vec<UserData>
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct UserData {
    id: Option<i32>,
    login: Option<String>,
    password_hash: Option<String>
}

impl From<&crate::models::User> for UserData {
    fn from(user: &crate::models::User) -> Self {
        Self {
            id: Some(user.id),
            login: Some(user.login.clone()),
            password_hash: Some(user.password_hash.clone())
        }
    }
}
