pub struct ServicePermissionRepository;

impl ServicePermissionRepository {
    /// ### Description
    /// Retrieve a single service permission from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ServicePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission is not found
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ServicePermissionRetrieveParams<'a>,
        database_client: A
    ) -> Option<crate::models::ServicePermission> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("SELECT id, service_id FROM service_permissions WHERE service_id = $1;")
            .bind(params.service_id)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Insert a single service permission to the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ServicePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission cannot be inserted because it violates the
    /// unique constraint
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ServicePermissionInsertParams<'a>,
        database_client: A
    ) -> Option<crate::models::ServicePermission> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("INSERT INTO service_permissions (service_id) VALUES ($1) RETURNING id, service_id;")
            .bind(params.service_id)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Delete a single service permission from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::ServicePermissionDeleteParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the permission is not found
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::ServicePermissionDeleteParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query("DELETE FROM service_permissions WHERE service_id = $1;")
            .bind(params.service_id)
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
