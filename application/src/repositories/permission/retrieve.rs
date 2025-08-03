use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn retrieve<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, client: C) -> Result<authios_domain::Permission> {
        use sqlx::query_as;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let result = query_as(sql)
            .bind(name)
            .fetch_one(&mut *client)
            .await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
