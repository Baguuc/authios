use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name already exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::PermissionCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::PermissionCreateError> {
        use crate::repositories::PermissionsRepository; 
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::PermissionCreateError as Error;
        
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

        // create permission
        {
            use crate::params::repository::PermissionInsertParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.name)
                .build()
                .unwrap();
                    
            PermissionsRepository::insert(params, &mut *client)
                .await
                .map_err(|_| Error::AlreadyExist)?; 
        }

        return Ok(());
    }
}
