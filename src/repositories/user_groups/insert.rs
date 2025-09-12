use crate::repositories::UserGroupsRepository;

impl UserGroupsRepository {
    /// # UserGroupsRepository::insert
    ///
    /// Grant user a group, inserting it to user_groups table in the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::user_groups::UserGroupInsertParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserGroupInsertParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let query = query(sql).bind(&params.user_login).bind(&params.group_name);

        let result = query.execute(&mut *client).await?;

        return Ok(result);
    }
}
