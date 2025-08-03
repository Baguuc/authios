use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<authios_domain::Permission> {
        use sqlx::query_as;

        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let result = query_as(sql).bind(name).fetch_one(client).await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
