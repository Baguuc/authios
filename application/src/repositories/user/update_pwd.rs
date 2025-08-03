use crate::prelude::*;

impl crate::UserRepository {
    pub async fn update_pwd<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, new_pwd: &String, client: C) -> Result<()> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "UPDATE users SET pwd = $1 WHERE login = $2;";
        let result = query(sql).bind(new_pwd).bind(login).execute(&mut *client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }  
}
