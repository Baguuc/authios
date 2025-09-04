impl crate::GroupsUseCase {
    /// # GroupsUseCase::grant
    ///
    /// grant a group to a user, checking for possible errors
    ///
    /// Errors:
    /// + when the group with provided name do not exist;
    /// + when the user with provided name do not exist;
    /// + when the user with provided login already has provided group;
    /// + when the user is not authorized for this operation;
    /// + when database connection cannot be acquired;
    ///
    pub async fn grant<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, token: &String, encoding_key: &String, client: A) -> Result<(), GroupGrantError> {
        type Error = GroupGrantError;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        match crate::UsersUseCase::check_permission(token, encoding_key, &String::from("authios:root:write"), &mut *client).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        let _ = crate::UsersRepository::retrieve(user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        crate::UserGroupsRepository::insert(user_login, group_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupGrantError {
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("UNAUTHORIZED_EXIST")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
