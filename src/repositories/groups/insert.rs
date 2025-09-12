use crate::repositories::GroupsRepository;

impl GroupsRepository {
    /// # GroupsRepository::insert
    ///
    /// Insert a group into the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::groups::GroupInsertParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
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
