use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::update
    ///
    /// update user's password data apart from login (PASSWORD UNHASHED!)
    ///
    pub async fn update<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(user_login: &String, new_pwd: &String, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "UPDATE users SET pwd = $2 WHERE login = $1;";
        query(sql)
            .bind(user_login)
            .bind(new_pwd)
            .execute(&mut *client)
            .await?;
        
        return Ok(());
    }
}
