use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// create a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name already exist;
    /// + when database connection cannot be acquired;
    /// + when user is not authorized to do this operation;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::GroupCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupCreateError> {
        use crate::repositories::GroupsRepository;
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::GroupCreateError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_token(params.token)
                .set_encryption_key(params.encryption_key)
                .set_permission_name(String::from("authios:all"))
                .build()
                .unwrap();
            
            match UsersUseCase::authorize(params, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }

        // insert group
        {
            use crate::params::repository::GroupInsertParamsBuilder as ParamsBuilder; 
            
            let params = ParamsBuilder::new()
                .set_name(params.name)
                .build()
                .unwrap();

            GroupsRepository::insert(params, &mut *client)
                .await
                .map_err(|_| Error::AlreadyExist)?;
        }
        
        return Ok(());
    }
}
