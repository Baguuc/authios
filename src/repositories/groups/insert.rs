use crate::repositories::GroupsRepository;

impl GroupsRepository {
    /// # GroupsRepository::insert
    ///
    /// insert a group into the database
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::GroupInsertParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO groups (name) VALUES ($1);";
        let query = query(sql).bind(params.name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
