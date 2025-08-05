impl crate::UsersRepository {
    /// # UsersRepository::insert
    ///
    /// insert a user into database with UNHASHED PASSWORD!
    ///
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(data: &authios_domain::User, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "INSERT INTO users (login, pwd) VALUES ($1, $2);";
        query(sql)
            .bind(&data.login)
            .bind(&data.pwd)
            .execute(&mut *client)
            .await?;
        
        return Ok(());
    }
}
