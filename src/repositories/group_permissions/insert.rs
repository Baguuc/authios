use crate::repositories::GroupPermissionsRepository;

impl GroupPermissionsRepository {
    /// # GroupPermissionsRepository::insert
    ///
    /// grant group a permission, inserting it into group_permission table in the database
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::GroupPermissionInsertParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO group_permissions (group_name, permission_name) VALUES ($1, $2);";
        let query = query(sql).bind(&params.group_name).bind(&params.permission_name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
