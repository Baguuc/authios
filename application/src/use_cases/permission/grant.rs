impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::grant
    ///
    /// grant a permission to a group, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when a group with provided name do not exist;
    /// + when a group with provided name already has provided permission;
    /// + when database connection cannot be acquired;
    ///
    pub async fn grant<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(permission_name: &String, group_name: &String, client: A) -> Result<(), PermissionGrantError> {
        type Error = PermissionGrantError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::PermissionsRepository::retrieve(permission_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        crate::GroupPermissionsRepository::insert(group_name, permission_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::Generic)?;
        
        return Ok(());
    }
}

pub enum PermissionGrantError {
    Generic
}

impl ToString for PermissionGrantError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
