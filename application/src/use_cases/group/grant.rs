impl crate::GroupsUseCase {
    /// # GroupsUseCase::grant
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
    pub async fn grant<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::PermissionGrantParams,
        client: A
    ) -> Result<(), GroupPermissionGrantError> {
        type Error = GroupPermissionGrantError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        match crate::UsersUseCase::check_permission(&params.auth.token, &params.auth.encoding_key, &String::from("authios:root:write"), &mut *client).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::PermissionsRepository::retrieve(&params.name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;
        
        let _ = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        crate::GroupPermissionsRepository::insert(&params.group_name, &params.name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupPermissionGrantError {
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
