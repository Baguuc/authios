pub struct UserResourcePermissionRepository;

impl UserResourcePermissionRepository {
    /// ### Description
    /// List resource permissions of specified type from specified service assigned to a user
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserResourcePermissionListParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns a list of all resource permission with their permission names and resource ids
    /// assigen to user that match provided criteria
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserResourcePermissionListParams<'a>,
        database_client: A
    ) -> Vec<crate::models::UserResourcePermission> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let sql = "SELECT
            rp.service_id,
            rp.resource_type,
            urp.resource_id,
            ARRAY_REMOVE(ARRAY_AGG(rp.permission_name), NULL) AS permissions
        FROM
          user_resource_permissions urp
        INNER JOIN
          resource_permissions rp
        ON
            urp.resource_permission_id = rp.id
        WHERE 
            urp.user_id = $1 AND
            rp.service_id = $2 AND
            rp.resource_type = $3
        GROUP BY rp.service_id, rp.resource_type, urp.resource_id
        ;";
        let result = sqlx::query_as(sql)
            .bind(params.user_id)
            .bind(params.service_id)
            .bind(params.resource_type)
            .fetch_all(&mut *database_client)
            .await
            .unwrap_or(vec![]);

        return result;
    }
    
    /// ### Description
    /// Retrieve resource permissions of specified type from specified service assigned to a user
    /// with specified resource id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserResourcePermissionRetrieveParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an option where None means no record matching provided criteria was found in the
    /// database
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserResourcePermissionRetrieveParams<'a>,
        database_client: A
    ) -> Option<crate::models::UserResourcePermission> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let sql = "SELECT
            rp.service_id,
            rp.resource_type,
            urp.resource_id,
            ARRAY_REMOVE(ARRAY_AGG(rp.permission_name), NULL) AS permissions
        FROM
          user_resource_permissions urp
        INNER JOIN
          resource_permissions rp
        ON
            urp.resource_permission_id = rp.id
        WHERE 
            urp.user_id = $1 AND
            rp.service_id = $2 AND
            rp.resource_type = $3 AND
            urp.resource_id = $4
        GROUP BY rp.service_id, rp.resource_type, urp.resource_id
        ;";
        let result = sqlx::query_as(sql)
            .bind(params.user_id)
            .bind(params.service_id)
            .bind(params.resource_type)
            .bind(params.resource_id)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        return result;
    }
    
    /// ### Description
    /// Insert a user's resource permission with a specified resource id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserResourcePermissionInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an option where None means the record couldn't be added as it violated the
    /// unique constraints of the table 
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserResourcePermissionInsertParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let result = sqlx::query("INSERT INTO user_resource_permissions (user_id, resource_permission_id, resource_id) VALUES ($1, $2, $3);")
            .bind(params.user_id)
            .bind(params.resource_permission_id)
            .bind(params.resource_id)
            .execute(&mut *database_client)
            .await;
        
        if result.is_ok() {
            Some(())
        } else {
            None
        }
    }
    
    /// ### Description
    /// Delete a user's resource permission with a specified resource id
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserResourcePermissionDeleteParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns an Option where None means no record matching provided criteria was found 
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserResourcePermissionDeleteParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let result = sqlx::query("DELETE FROM user_resource_permissions WHERE user_id = $1 AND resource_permission_id = $2 AND resource_id = $3;")
            .bind(params.user_id)
            .bind(params.resource_permission_id)
            .bind(params.resource_id)
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
