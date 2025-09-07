use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
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
        params: crate::params::GroupCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupCreateError> {
        use crate::repositories::GroupsRepository; 
        use crate::errors::use_case::GroupCreateError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        GroupsRepository::insert(&params.name, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}
