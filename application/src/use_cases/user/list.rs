impl crate::UsersUseCase {
    /// # UsersUseCase::list
    ///
    /// list users
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(client: A) -> Result<Vec<authios_domain::User>, UserListError> {
        type Error = UserListError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let data = crate::UsersRepository::list(&mut *client)
            .await
            .unwrap_or(vec![]);
        
        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserListError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
