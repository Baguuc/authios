impl crate::PermissionsRepository {
    /// # PermissionsRepository::insert
    ///
    /// insert a permission into the database
    ///
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(data: &authios_domain::Permission, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO permissions (name) VALUES ($1);";
        let query = query(sql).bind(&data.name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
