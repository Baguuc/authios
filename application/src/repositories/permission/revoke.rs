use crate::prelude::*;

impl crate::PermissionRepository {
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let result = query(sql).bind(group_name).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
