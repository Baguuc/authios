use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission and add it to the root group, checking for possible errors
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
        use crate::repositories::{
            PermissionsRepository,
            GroupPermissionsRepository
        }; 
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::PermissionCreateError as Error;
        
        let mut tx = client.begin()
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
            
            match UsersUseCase::authorize(params, &mut *tx).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }

        // create permission
        {
            use crate::params::repository::PermissionInsertParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.name.clone())
                .build()
                .unwrap();
                    
            PermissionsRepository::insert(params, &mut *tx)
                .await
                .map_err(|_| Error::AlreadyExist)?; 
        }
        
        // grant the root group
        {
            use crate::params::repository::GroupPermissionInsertParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_group_name(String::from("root"))
                .set_permission_name(params.name)
                .build()
                .unwrap();
                    
            GroupPermissionsRepository::insert(params, &mut *tx)
                .await
                .map_err(|_| Error::AlreadyExist)?; 
            
        }

        let _ = tx.commit().await;

        return Ok(());
    }
}
