pub struct ResourcePermissionRepository;

impl ResourcePermissionRepository {
    /// ### Description
    /// Retrieve a single resource permission from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ResourcePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission cannot is not found
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ResourcePermissionRetrieveParams<'a>,
        database_client: A
    ) -> Option<crate::models::ResourcePermission> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("SELECT id, service_id, resource_type, permission_name FROM resource_permissions WHEREE service_id = $1 AND resource_type = $2 AND permission_name = $3;")
            .bind(params.service_id)
            .bind(params.resource_type)
            .bind(params.permission_name)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Insert a single resource permission to the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ResourcePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission cannot be inserted because it violates the
    /// unique constraint
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ResourcePermissionInsertParams<'a>,
        database_client: A
    ) -> Option<crate::models::ResourcePermission> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("INSERT INTO resource_permissions (service_id, resource_type, permission_name) VALUES ($1, $2, $3) RETURNING id, service_id, resource_type, permission_name;")
            .bind(params.service_id)
            .bind(params.resource_type)
            .bind(params.permission_name)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Delete a single resource permission from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ResourcePermissionDeleteParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission is not found
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ResourcePermissionDeleteParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query("DELETE FROM resource_permissions WHERE service_id = $1 AND resource_type = $2 AND permission_name = $3;")
            .bind(params.service_id)
            .bind(params.resource_type)
            .bind(params.permission_name)
            .execute(&mut *database_client)
            .await
            .unwrap();

        if result.rows_affected() > 0 {
            Some(())
        } else {
            None
        }
    }   
}
