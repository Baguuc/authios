use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM groups WHERE name = $1;";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }    
}
