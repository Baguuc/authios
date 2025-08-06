impl crate::UsersUseCase {
    /// # UsersUseCase::sync
    ///
    /// sync users from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'c, A: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::User>, client: A) -> Result<(), UserSyncError> {
        type Error = UserSyncError; 

        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let old = Self::list(&mut *client)
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for user in changes.delete {
            let _ = Self::delete(&user.login, &mut *client)
                .await
                .map_err(|_| Error::DatabaseConnection)?;
        }

        for user in changes.create {
            let _ = Self::register(&user, &mut *client)
                .await
                .map_err(|_| Error::DatabaseConnection)?;

            for group in user.groups {
                 let _ = crate::GroupsUseCase::grant(&user.login, &group, &mut *client)
                     .await
                     .map_err(|_| Error::DatabaseConnection)?;
            }
        }
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserSyncError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
