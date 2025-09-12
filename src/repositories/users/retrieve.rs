use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::retrieve
    ///
    /// Retrieve user identified by login from the database
    ///
    /// ### Arguments:
    /// + params: [crate::params::repository::users::UserRetrieveParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    ///
    pub async fn retrieve<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserRetrieveParams,
        client: C
    ) -> Result<crate::models::User, sqlx::Error> {
        use sqlx::query_as;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "SELECT
          u.login,
          u.pwd,
          ARRAY_REMOVE(ARRAY_AGG(ug.group_name), NULL) AS groups
        FROM 
          users u
        LEFT JOIN 
          user_groups ug 
          ON 
          ug.user_login = u.login
        WHERE
            u.login = $1
        GROUP BY
          u.login, u.pwd;
        ;";
        let row = query_as(sql).bind(&params.login).fetch_one(&mut *client).await?;
        
        return Ok(row);
    }
}
