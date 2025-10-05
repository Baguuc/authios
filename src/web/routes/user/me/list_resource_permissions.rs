#[actix_web::get("/permissions/resource/{service_id}/{resource_type}")]
pub async fn controller(
    token: crate::web::extractors::TokenExtractor,
    path: actix_web::web::Path<Path>,
    query: actix_web::web::Query<Query>,
    config: actix_web::web::Data<crate::config::Config>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use serde_json::{json,Map,Value,Number};
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
            let mut data_map = Map::new();

            // there is 5 entries max so we don't worry about performance
            let permissions = data.permissions.iter()
                .map(|row| {
                    let mut data_map = Map::new();
                    
                    if query.get_service_id.unwrap_or(true) {
                        data_map.insert(String::from("service_id"), Value::String(row.service_id.clone()));
                    }
                    
                    if query.get_resource_type.unwrap_or(true) {
                        data_map.insert(
                            String::from("resource_type"),
                            Value::String(row.resource_type.clone())
                        );
                    }
                    
                    if query.get_resource_id.unwrap_or(true) {
                        data_map.insert(
                            String::from("resource_id"),
                            Value::Number(Number::from(row.resource_id))
                        );
                    }
                    
                    if query.get_permission_names.unwrap_or(true) {
                        data_map.insert(
                            String::from("permissions"),
                            Value::Array(
                                row.permissions.iter().map(|v| Value::String(v.to_string())).collect::<Vec<Value>>()
                            )
                        );
                    }

                    Value::Object(data_map)
                })
                .collect::<Vec<Value>>();
            
            data_map.insert(
                String::from("permissions"),
                Value::Array(permissions)
            );
            
            if query.get_page_number.unwrap_or(true) {
                data_map.insert(
                    String::from("page_number"),
                    Value::Number(Number::from(data.page_number))
                );
            }

            if query.get_total_page_count.unwrap_or(true) {
                data_map.insert(
                    String::from("total_page_count"),
                    Value::Number(Number::from(data.total_page_count))
                );
            }

            HttpResponse::Ok()
                .json(json!(data_map))
        },
        
        Err(error) => match error {
            Error::InvalidToken => HttpResponse::BadRequest()
                .json(json!({ "code": "invalid_token" })),
        }
    }
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
