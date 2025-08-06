impl crate::UsersUseCase {
    /// # UsersUseCase::sync
    ///
    /// sync users from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when password of any user to add cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'c, A: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::User>, client: A) -> Result<(), UserSyncError> {
        use crate::UsersRepository; 
        use crate::UserGroupsRepository; 

        type Error = UserSyncError; 

        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // won't error
        let old = UsersRepository::list(&mut *client)
            .await
            .unwrap();

        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for user in changes.delete {
            let _ = UsersRepository::delete(&user.login, &mut *client).await;
        }

        for mut user in changes.create {
            user.pwd = crate::utils::hash_password(user.pwd)
                .map_err(|_| Error::CannotHash(user.login.clone()))?;

            let _ = UsersRepository::insert(&user, &mut *client).await;

            for group_name in user.groups {
                 let _ = UserGroupsRepository::insert(&user.login, &group_name, &mut *client)
                     .await
                     .map_err(|_| Error::GroupNotExist(group_name))?;
            }
        }
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserSyncError {
    #[error("PERMISSION_NOT_EXIST:{0}")]
    GroupNotExist(String),
    #[error("CANNOT_HASH:{0}")]
    CannotHash(String),
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
