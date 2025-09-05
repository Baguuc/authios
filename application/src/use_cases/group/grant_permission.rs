impl crate::GroupsUseCase {
    /// # GroupsUseCase::grant_permission
    ///
    /// grant a permission to a group, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when a group with provided name do not exist;
    /// + when a group with provided name already has provided permission;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn grant_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::GroupGrantPermissionParams,
        client: A
    ) -> Result<(), authios_domain::GroupGrantPermissionError> {
        type Error = authios_domain::GroupGrantPermissionError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::PermissionsRepository::retrieve(&params.permission_name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;
        
        let _ = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        crate::GroupPermissionsRepository::insert(&params.group_name, &params.permission_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}
