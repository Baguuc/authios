use crate::repositories::GroupPermissionsRepository;

impl GroupPermissionsRepository {
    /// # GroupPermissionsRepository::delete
    ///
    /// Revoke group a permission, deleting it from group_permission table in the database.
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::group_permissions::GroupPermissionDeleteParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::GroupPermissionDeleteParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let query = query(sql).bind(&params.group_name).bind(&params.permission_name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
