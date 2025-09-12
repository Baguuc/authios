use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
    /// # PermissionsUseCase::delete
    ///
    /// Delete a permission, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::permission::PermissionDeleteParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::permission::PermissionDeleteError]
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
            use crate::params::use_case::UserAuthorizeParams as Params;
            
            match UsersUseCase::authorize(Params { token: params.token, encryption_key: params.encryption_key, permission_name: String::from("authios:all") }, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }
        
        // delete permission
        {
            use crate::params::repository::PermissionDeleteParams as Params;
            
            let result = PermissionsRepository::delete(Params { name: params.name }, &mut *client)
                .await
                .unwrap();

            if result.rows_affected() == 0 {
                return Err(Error::NotFound);
            }
        } 
        return Ok(());
    }
}
