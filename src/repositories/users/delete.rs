use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::delete
    ///
    /// delete a user identified by login
    ///
    pub async fn delete<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "DELETE FROM users WHERE login = $1;";
        query(sql)
            .bind(login)
            .execute(&mut *client)
            .await?;

        return Ok(());
    } 
}
