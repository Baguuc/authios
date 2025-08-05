impl crate::GroupsRepository {
    /// # GroupsRepository::insert
    ///
    /// insert a group into the database
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(data: &authios_domain::Group, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO groups (name) VALUES ($1);";
        let query = query(sql).bind(&data.name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
