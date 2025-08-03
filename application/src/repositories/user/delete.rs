use crate::prelude::*;

impl crate::UserRepository {
    pub async fn delete<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, client: C) -> Result<()> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM users WHERE login = $1;";
        let result = query(sql)
            .bind(login)
            .execute(&mut *client)
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    } 
}
