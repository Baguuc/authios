impl crate::GroupsUseCase {
    /// # GroupsUseCase::sync
    ///
    /// sync groups from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::Group>, client: C) -> Result<(), GroupSyncError> {        
        type Error = GroupSyncError; 

        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let old = Self::list(&mut *client)
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for group in changes.delete {
            let _ = Self::delete(&group.name, &mut *client)
                .await
                .map_err(|_| Error::DatabaseConnection)?;
        }

        for group in changes.create {
            let _ = Self::create(&group, &mut *client)
                .await
                .map_err(|_| Error::DatabaseConnection)?;

            for permission in group.permissions {
                 let _ = crate::PermissionsUseCase::grant(&permission, &group.name, &mut *client)
                     .await
                    .map_err(|_| Error::DatabaseConnection)?;
            }
        }
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupSyncError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
