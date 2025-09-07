use crate::repositories::UsersRepository;

impl UsersRepository {
    /// # UsersRepository::insert
    ///
    /// insert a user into database with UNHASHED PASSWORD!
    ///
    pub async fn insert<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, pwd: &String, client: C) -> Result<(), sqlx::Error> {
        use sqlx::query;
        
        let mut client = client
            .acquire()
            .await?;

        let sql = "INSERT INTO users (login, pwd) VALUES ($1, $2);";
        query(sql)
            .bind(login)
            .bind(pwd)
            .execute(&mut *client)
            .await?;
        
        return Ok(());
    }
}
