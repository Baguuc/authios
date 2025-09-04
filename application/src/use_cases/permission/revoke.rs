impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::revoke
    ///
    /// revoke a permission from a group, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when a group with provided name do not exist;
    /// + when a group with provided name didn't had provided permission;
    /// + when database connection cannot be acquired;
    ///
    pub async fn revoke<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::PermissionRevokeParams,
        client: A
    ) -> Result<(), PermissionRevokeError> {
        type Error = PermissionRevokeError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::PermissionsRepository::retrieve(&params.name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;
        
        let group = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        // not added yet
        if group.permissions.contains(&params.name) {
            return Err(Error::NotAddedYet);
        }
        
        // this won't error so we can skip this result
        let _ = crate::GroupPermissionsRepository::delete(&params.group_name, &params.name, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionRevokeError {
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
