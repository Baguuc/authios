impl crate::GroupsUseCase {
    /// # GroupsUseCase::list
    ///
    /// list groups
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(client: A) -> Result<Vec<authios_domain::Group>, GroupListError> {
        type Error = GroupListError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let data = crate::GroupsRepository::list(&mut *client)
            .await
            .unwrap_or(vec![]);
        
        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupListError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
