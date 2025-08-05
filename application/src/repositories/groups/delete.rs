impl crate::GroupsRepository {
    /// # GroupsRepository::retrieve
    ///
    /// delete a group identified by name in the database
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM groups WHERE name = $1;";
        let query = query(sql).bind(name);

        query.execute(&mut *client).await?;
        
        return Ok(());
    }
}
