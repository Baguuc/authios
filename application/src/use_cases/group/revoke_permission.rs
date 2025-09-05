impl crate::GroupsUseCase {
    /// # GroupsUseCase::revoke
    ///
    /// revoke a permission from a group, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when a group with provided name do not exist;
    /// + when a group with provided name didn't had provided permission;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn revoke<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::GroupRevokePermissionParams,
        client: A
    ) -> Result<(), authios_domain::GroupRevokePermissionError> {
        type Error = authios_domain::GroupRevokePermissionError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::PermissionsRepository::retrieve(&params.permission_name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;
        
        let group = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        // not added yet
        if group.permissions.contains(&params.permission_name) {
            return Err(Error::NotAddedYet);
        }
        
        // this won't error so we can skip this result
        let _ = crate::GroupPermissionsRepository::delete(&params.group_name, &params.permission_name, &mut *client)
            .await;
        
        return Ok(());
    }
}
