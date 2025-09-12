use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::delete
    ///
    /// Delete a user identified by login
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::users::UserDeleteParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn delete<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserDeleteParams,
        client: C
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM users WHERE login = $1;";
        let result = query(sql)
            .bind(&params.login)
            .execute(&mut *client)
            .await?;

        return Ok(result);
    } 
}
