use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn grant<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let result = query(sql)
            .bind(user_login)
            .bind(name)
            .execute(&mut *client)
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
