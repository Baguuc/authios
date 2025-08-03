use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn list<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(client: C) -> Result<Vec<authios_domain::Permission>> {
        use sqlx::query_as;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "SELECT * FROM permissions;";
        let result = query_as(sql)
            .fetch_all(&mut *client)
            .await;

        match result {
            Ok(data) => return Ok(data),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
