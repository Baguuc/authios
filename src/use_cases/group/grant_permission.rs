use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
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
        params: crate::params::GroupGrantPermissionParams,
        client: A
    ) -> Result<(), crate::errors::GroupGrantPermissionError> {
        use crate::repositories::{
            PermissionsRepository,
            GroupsRepository,
            GroupPermissionsRepository
        };
        use crate::errors::GroupGrantPermissionError as Error;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = PermissionsRepository::retrieve(&params.permission_name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;
        
        let _ = GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        GroupPermissionsRepository::insert(&params.group_name, &params.permission_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}
