pub enum AdminListUserResourcePermissionsResponse {
    Ok(OkData),
    UserNotFound,
    Unauthorized
}

impl AdminListUserResourcePermissionsResponse { pub fn partialize_ok(
        self: Self,
        save_page_number: bool,
        save_total_page_count: bool,
        save_service_id: bool,
        save_resource_type: bool,
        save_resource_id: bool,
        save_permission_names: bool
    ) -> Self {
        match self {
            Self::Ok(data) => {
                // there is 5 entries max so we don't worry about performance and time of loop
                // execution
                let permissions = data.permissions.iter()
                    .map(|e| {
                        UserPermissionData {         
                            service_id: if save_service_id 
                                { e.service_id.clone() } else { None },
                            resource_type: if save_resource_type 
                                { e.resource_type.clone() } else { None },
                            resource_id: if save_resource_id
                                { e.resource_id } else { None },
                            permissions: if save_permission_names
                                { e.permissions.clone() } else { None },
                        }
                    })
                    .collect::<Vec<UserPermissionData>>();

                let ok_data = OkData { 
                    page_number: if save_page_number
                        { data.page_number } else { None },
                    total_page_count: if save_total_page_count
                        { data.total_page_count } else { None },

                    permissions
                };

                Self::Ok(ok_data)
            },
            _ => self
        }
    }
}

impl From<Result<crate::models::UserResourcePermissionPage, crate::errors::use_case::AdminListUserResourcePermissionsError>> for AdminListUserResourcePermissionsResponse {
    fn from(result: Result<crate::models::UserResourcePermissionPage, crate::errors::use_case::AdminListUserResourcePermissionsError>) -> Self {
        use crate::errors::use_case::AdminListUserResourcePermissionsError as Error;

        match result {
            Ok(page) => {
                Self::Ok(OkData {
                    page_number: Some(page.page_number),
                    total_page_count: Some(page.total_page_count),
                    permissions: page.permissions.iter().map(|e| e.into()).collect()
                })
            },
            Err(error) => match error {
                Error::NotFound => Self::UserNotFound,
                Error::Unauthorized => Self::Unauthorized,
            }
        }
    }
}

impl Into<actix_web::HttpResponse> for AdminListUserResourcePermissionsResponse {
    fn into(self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use serde_json::json;

        match self {
            Self::Ok(data) => HttpResponse::Ok()
                .json(json!(data)),
            
            Self::UserNotFound => HttpResponse::NotFound()
                .json(json!({ "code": "user_not_found" })),
            
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
    permissions: Vec<UserPermissionData>
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct UserPermissionData {
    service_id: Option<String>,
    resource_type: Option<String>,
    resource_id: Option<i32>,
    permissions: Option<Vec<String>>,
}

impl From<&crate::models::UserResourcePermission> for UserPermissionData {
    fn from(permission: &crate::models::UserResourcePermission) -> Self {
        Self {        
            service_id: Some(permission.service_id.clone()),
            resource_type: Some(permission.resource_type.clone()),
            resource_id: Some(permission.resource_id.clone()),
            permissions: Some(permission.permissions.clone()),
        }
    }
}
