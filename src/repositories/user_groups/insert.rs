use crate::repositories::UserGroupsRepository;

impl UserGroupsRepository {
    /// # UserGroupsRepository::insert
    ///
    /// grant user a group, inserting it to user_groups table in the database
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, client: A) -> Result<(), sqlx::Error> {
        use sqlx::query;

        let mut client = client
            .acquire()
            .await?;
        
        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let query = query(sql).bind(user_login).bind(group_name);

        query.execute(&mut *client).await?;

        return Ok(());
    }
}
