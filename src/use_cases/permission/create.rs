use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission and add it to the root group, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::permission::PermissionCreateParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::permission::PermissionCreateError]
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
            use crate::params::use_case::UserAuthorizeParams as Params;

            let params = Params {
                encryption_key: params.encryption_key,
                permission_name: String::from("authios:all"),
                token: params.token,
            };
            
            match UsersUseCase::authorize(params, &mut *tx).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }

        // create permission
        {
            use crate::params::repository::PermissionInsertParams as Params;
                    
            PermissionsRepository::insert(Params { name: params.name.clone() }, &mut *tx)
                .await
                .map_err(|_| Error::AlreadyExist)?; 
        }
        
        // grant to the root group
        {
            use crate::params::repository::GroupPermissionInsertParams as Params;
                    
            GroupPermissionsRepository::insert(Params { group_name: String::from("root"), permission_name: params.name }, &mut *tx)
                .await
                .map_err(|_| Error::AlreadyExist)?; 
            
        }

        let _ = tx.commit().await;

        return Ok(());
    }
}
