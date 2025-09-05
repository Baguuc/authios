impl crate::GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// create a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name already exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::GroupCreateParams,
        client: A
    ) -> Result<(), authios_domain::GroupCreateError> {
        type Error = authios_domain::GroupCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        crate::GroupsRepository::insert(&params.name, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}
