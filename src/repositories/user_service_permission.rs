pub struct UserServicePermissionRepository;

impl UserServicePermissionRepository {
    /// ### Description
    /// Retrieve service permissions of specified type from specified service assigned to a user
    /// with specified service id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserServicePermissionRetrieveParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an option where None means no record matching provided criteria was found in the
    /// database
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserServicePermissionRetrieveParams<'a>,
        database_client: A
    ) -> Option<crate::models::ServicePermission> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let sql = "SELECT sp.id, sp.service_id FROM user_service_permissions usp INNER JOIN sp ON usp.service_permission_id = sp.id WHERE sp.id = $1";
        let result = sqlx::query_as(sql)
            .bind(params.permission_id)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        return result;
    }
    
    /// ### Description
    /// Insert a user's service permission with a specified resource id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserServicePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an option where None means the record couldn't be added as it violated the
    /// unique constraints of the table 
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserServicePermissionInsertParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let result = sqlx::query("INSERT INTO user_service_permissions (user_id, service_permission_id) VALUES ($1, $2);")
            .bind(params.user_id)
            .bind(params.permission_id)
            .execute(&mut *database_client)
            .await;
        
        if result.is_ok() {
            Some(())
        } else {
            None
        }
    }
    
    /// ### Description
    /// Delete a user's service permission with a specified resource id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserServicePermissionDeleteParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an Option where None means no record matching provided criteria was found 
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserServicePermissionDeleteParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let result = sqlx::query("DELETE FROM user_service_permissions WHERE user_id = $1 AND service_permission_id = $2;")
            .bind(params.user_id)
            .bind(params.permission_id)
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
