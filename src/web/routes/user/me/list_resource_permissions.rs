#[actix_web::get("/permissions/resource/{service_id}/{resource_type}")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    path: actix_web::web::Path<Path>,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::json;
    use actix_web::HttpResponse;
    use crate::params::use_case::UserListResourcePermissionsParams as Params;
    use crate::errors::use_case::UserListResourcePermissionsError as Error;
    use crate::use_cases::UserUseCase as UseCase;

    let mut database_client = database_client
        .into_inner()
        .acquire()
        .await
        .unwrap();

    let params = Params {
        token: &token.0,
        jwt_encryption_key: &config.jwt.encryption_key,
        service_id: &path.service_id,
        resource_type: &path.resource_type,
        page_number: &query.page_number.unwrap_or(0)
    };

    match UseCase::list_resource_permissions(params, &mut *database_client).await {
        Ok(data) => {
            // there is 5 entries max so we don't worry about performance and time of loop
            // execution
            let permissions = data.permissions.iter()
                .map(|e| {
                    ResponseUserResourcePermission {         
                        service_id: if query.get_service_id.unwrap_or(true) 
                            { Some(e.service_id.clone()) } else { None },
                        resource_type: if query.get_resource_type.unwrap_or(true) 
                            { Some(e.resource_type.clone()) } else { None },
                        resource_id: if query.get_resource_id.unwrap_or(true)
                            { Some(e.resource_id) } else { None },
                        permissions: if query.get_permission_names.unwrap_or(true)
                            { Some(e.permissions.clone()) } else { None },
                    }
                })
                .collect::<Vec<ResponseUserResourcePermission>>();

            let response = Response {
                page_number: if query.get_page_number.unwrap_or(true)
                    { Some(data.page_number) } else { None },
                total_page_count: if query.get_total_page_count.unwrap_or(true)
                    { Some(data.total_page_count) } else { None },
                permissions
            };

            HttpResponse::Ok()
                .json(json!(response))
        },
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct Response {
    page_number: Option<u32>,
    total_page_count: Option<u32>,
    permissions: Vec<ResponseUserResourcePermission>
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
struct ResponseUserResourcePermission {
    service_id: Option<String>,
    resource_type: Option<String>,
    resource_id: Option<i32>,
    permissions: Option<Vec<String>>
}

#[derive(serde::Deserialize)]
struct Path {
    service_id: String,
    resource_type: String
}

#[derive(serde::Deserialize)]
struct Query {
    page_number: Option<u32>,
    get_page_number: Option<bool>,
    get_total_page_count: Option<bool>,
    get_service_id: Option<bool>,
    get_resource_type: Option<bool>,
    get_resource_id: Option<bool>,
    get_permission_names: Option<bool>
}
