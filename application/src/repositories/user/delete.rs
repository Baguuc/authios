use crate::prelude::*;

impl crate::UserRepository {
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM users WHERE login = $1;";
        let result = query(sql).bind(login).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    } 
}
