use crate::repositories::PermissionsRepository;

impl PermissionsRepository {
    /// # PermissionsRepository::retrieve
    ///
    /// delete a permission identified by name in the database
    ///
    pub async fn delete<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::PermissionDeleteParams,
        client: C
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM permissions WHERE name = $1;";
        let query = query(sql).bind(&params.name);

        let result = query.execute(&mut *client).await?;
        
        return Ok(result);
    }
}
