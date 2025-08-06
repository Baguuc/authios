impl crate::UsersUseCase {
    /// # UsersUseCase::delete
    ///
    /// delete a user, checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(login: &String, client: A) -> Result<(), UserDeleteError> {
        type Error = UserDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::UsersRepository::retrieve(login, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        // this won't error so we can skip this result
        let _ = crate::UsersRepository::delete(login, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserDeleteError {
    #[error("USER_NOT_EXIST")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
