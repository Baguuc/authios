use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO groups (name) VALUES ($1);";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
