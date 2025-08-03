use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn revoke<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let result = query(sql)
            .bind(group_name)
            .bind(name)
            .execute(&mut *client)
            .await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
