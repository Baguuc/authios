use crate::prelude::*;

impl crate::UserRepository {
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<authios_domain::User>> {
        use sqlx::query_as;

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
        GROUP BY
          u.login, u.pwd
        ;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
