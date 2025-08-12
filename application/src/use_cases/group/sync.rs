impl crate::GroupsUseCase {
    /// # GroupsUseCase::sync
    ///
    /// sync groups from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::Group>, client: C) -> Result<(), GroupSyncError> {        
        use crate::GroupsRepository; 
        use crate::GroupPermissionsRepository; 

        type Error = GroupSyncError; 

        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // won't error
        let old = GroupsRepository::list(&mut *client)
            .await
            .unwrap();
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for group in changes.delete {
            let _ = GroupsRepository::delete(&group.name, &mut *client).await;
        }

        for group in changes.create {
            let _ = GroupsRepository::insert(&group.name, &mut *client).await;

            for permission_name in group.permissions {
                 let _ = GroupPermissionsRepository::insert(&group.name, &permission_name, &mut *client)
                     .await
                     .map_err(|_| Error::PermissionNotExist(permission_name));
            }
        }
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupSyncError {
    #[error("PERMISSION_NOT_EXIST:{0}")]
    PermissionNotExist(String),
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
