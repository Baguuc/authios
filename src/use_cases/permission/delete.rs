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
        params: crate::params::PermissionDeleteParams,
        client: A
    ) -> Result<(), crate::errors::PermissionDeleteError> {
        use crate::repositories::PermissionsRepository;
        use crate::errors::PermissionDeleteError as Error;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
                
        let _ = PermissionsRepository::retrieve(&params.name, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        // this won't error so we can skip this result
        let _ = PermissionsRepository::delete(&params.name, &mut *client)
            .await;
        
        return Ok(());
    }
}
