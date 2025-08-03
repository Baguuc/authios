use crate::prelude::*;

impl crate::GroupRepository {
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let result = query(sql).bind(user_login).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
