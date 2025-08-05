impl crate::PermissionsRepository {
    /// # PermissionsRepository::retrieve
    ///
    /// delete a permission identified by name in the database
    ///
    pub async fn delete<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM permissions WHERE name = $1;";
        let query = query(sql).bind(name);

        query.execute(&mut *client).await?;
        
        return Ok(());
    }
}
