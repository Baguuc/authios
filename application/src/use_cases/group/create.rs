impl crate::GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// create a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name already exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(data: &authios_domain::Group, client: A) -> Result<(), GroupCreateError> {
        type Error = GroupCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        crate::GroupsRepository::insert(data, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
