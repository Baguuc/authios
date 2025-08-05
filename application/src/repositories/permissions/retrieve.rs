impl crate::PermissionsRepository {
    /// # PermissionsRepository::retrieve
    ///
    /// retrieve a permission identified by name from the database
    ///
    pub async fn retrieve<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, client: C) -> Result<authios_domain::Permission, sqlx::Error> {
        use sqlx::query_as;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let query = query_as(sql).bind(name);

        let row = query.fetch_one(&mut *client).await?;

        return Ok(row);
    }
}
