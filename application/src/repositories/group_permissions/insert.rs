impl crate::GroupPermissionsRepository {
    /// # GroupPermissionsRepository::insert
    ///
    /// grant group a permission, inserting it into group_permission table in the database
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(group_name: &String, permission_name: &String, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO group_permissions (group_name, permission_name) VALUES ($1, $2);";
        let query = query(sql).bind(group_name).bind(permission_name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
