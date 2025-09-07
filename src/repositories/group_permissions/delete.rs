use crate::repositories::GroupPermissionsRepository;

impl GroupPermissionsRepository {
    /// # GroupPermissionsRepository::delete
    ///
    /// revoke group a permission, deleting it from group_permission table in the database
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(group_name: &String, permission_name: &String, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let query = query(sql).bind(group_name).bind(permission_name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
