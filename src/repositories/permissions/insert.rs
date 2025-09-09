use crate::repositories::PermissionsRepository;

impl PermissionsRepository {
    /// # PermissionsRepository::insert
    ///
    /// insert a permission into the database
    ///
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::PermissionInsertParams,
        client: C
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO permissions (name) VALUES ($1);";
        let query = query(sql).bind(&params.name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
