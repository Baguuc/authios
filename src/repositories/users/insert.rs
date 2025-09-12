use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::insert
    ///
    /// insert a user into database with UNHASHED PASSWORD!
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::users::UserInsertParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserInsertParams,
        client: C
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "INSERT INTO users (login, pwd) VALUES ($1, $2);";
        let result = query(sql)
            .bind(&params.login)
            .bind(&params.pwd)
            .execute(&mut *client)
            .await?;
        
        return Ok(result);
    }
}
