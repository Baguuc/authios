use crate::repositories::GroupsRepository;

impl GroupsRepository {
    /// # GroupsRepository::delete
    ///
    /// Delete a group identified by name in the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::groups::GroupDeleteParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::GroupDeleteParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM groups WHERE name = $1;";
        let query = query(sql).bind(&params.name);

        let result = query.execute(&mut *client).await?;
        
        return Ok(result);
    }
}
