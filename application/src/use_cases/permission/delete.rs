impl crate::PermissionsUseCase {
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
        params: authios_domain::PermissionDeleteParams,
        client: A
    ) -> Result<(), authios_domain::PermissionDeleteError> {
        type Error = authios_domain::PermissionDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
                
        let _ = crate::PermissionsRepository::retrieve(&params.name, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        // this won't error so we can skip this result
        let _ = crate::PermissionsRepository::delete(&params.name, &mut *client)
            .await;
        
        return Ok(());
    }
}
