impl crate::UsersRepository {
    /// # UsersRepository::retrieve
    ///
    /// retrieve user identified by login from the database
    ///
    pub async fn retrieve<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, client: C) -> Result<authios_domain::User, sqlx::Error> {
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
        let row = query_as(sql).bind(login).fetch_one(&mut *client).await?;
        
        return Ok(row);
    }
}
