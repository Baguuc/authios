impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::sync
    ///
    /// sync permissions from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(new: Vec<String>, client: A) -> Result<(), PermissionSyncError> {
        use crate::PermissionsRepository;
        
        type Error = PermissionSyncError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
                
        // won't error
        let old = PermissionsRepository::list(&mut *client)
            .await
            .unwrap()
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);

        for permission_name in changes.delete {
            let _ = PermissionsRepository::delete(&permission_name, &mut *client).await;
        }

        for permission_name in changes.create {
            let permission = authios_domain::Permission {
                name: permission_name
            };

            let _ = PermissionsRepository::insert(&permission, &mut *client).await;
        }
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionSyncError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
