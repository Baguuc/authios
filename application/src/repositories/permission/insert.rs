use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO permissions (name) VALUES ($1);";
        let result = query(sql)
            .bind(name)
            .execute(&mut *client)
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
