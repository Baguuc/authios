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
        params: crate::params::use_case::GroupCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupCreateError> {
        use crate::repositories::GroupsRepository; 
        use crate::errors::use_case::GroupCreateError as Error; 
        use crate::params::repository::GroupInsertParamsBuilder as ParamsBuilder; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let params = ParamsBuilder::new()
            .set_name(params.name)
            .build()
            .unwrap();

        GroupsRepository::insert(params, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}
