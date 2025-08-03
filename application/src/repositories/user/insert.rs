use crate::prelude::*;

impl crate::UserRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO users (login, pwd) VALUES ($1, $2);";
        let result = query(sql).bind(login).bind(Self::hash_password(pwd.clone())?).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
