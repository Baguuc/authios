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
    pub async fn revoke<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(permission_name: &String, group_name: &String, client: A) -> Result<(), PermissionRevokeError> {
        type Error = PermissionRevokeError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::PermissionsRepository::retrieve(permission_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let group = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        // not added yet
        if group.permissions.contains(permission_name) {
            return Err(Error::Generic);
        }
        
        // this won't error so we can skip this result
        let _ = crate::GroupPermissionsRepository::delete(group_name, permission_name, &mut *client)
            .await;
        
        return Ok(());
    }
}

pub enum PermissionRevokeError {
    Generic
}

impl ToString for PermissionRevokeError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
