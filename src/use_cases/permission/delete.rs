use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
    /// # PermissionsUseCase::delete
    ///
    /// delete a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::PermissionDeleteParams,
        client: A
    ) -> Result<(), crate::errors::use_case::PermissionDeleteError> {
        use crate::repositories::PermissionsRepository;
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::PermissionDeleteError as Error;

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
        
        // delete permission
        {
            use crate::params::repository::PermissionDeleteParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.name)
                .build()
                .unwrap();
            
            let result = PermissionsRepository::delete(params, &mut *client)
                .await
                .unwrap();

            if result.rows_affected() == 0 {
                return Err(Error::NotExist);
            }
        } 
        return Ok(());
    }
}
