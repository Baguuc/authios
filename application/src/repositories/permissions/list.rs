impl crate::PermissionsRepository {
    /// # PermissionsRepository::list
    ///
    /// list all permissions in the database
    ///
    pub async fn list<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(client: C) -> Result<Vec<authios_domain::Permission>, sqlx::Error> {
        use sqlx::query_as;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "SELECT * FROM permissions;";
        let query = query_as(sql);

        let rows = query.fetch_all(&mut *client).await?;
        
        return Ok(rows);
    }
}
