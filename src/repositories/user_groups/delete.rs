use crate::repositories::UserGroupsRepository;

impl UserGroupsRepository {
    /// # UserGroupsRepository::delete
    ///
    /// revoke user a group, deleting it from user_groups table in the database
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "DELETE FROM user_groups WHERE user_login = $1 AND group_name = $2;";
        let query = query(sql).bind(user_login).bind(group_name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
