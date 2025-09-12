use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::update
    ///
    /// Update user's data apart from login (PASSWORD UNHASHED!)
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::users::UserUpdateParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn update<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserUpdateParams,
        client: C
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "UPDATE users SET pwd = $2 WHERE login = $1;";
        let result = query(sql)
            .bind(&params.user_login)
            .bind(&params.new_pwd)
            .execute(&mut *client)
            .await?;
        
        return Ok(result);
    }
}
