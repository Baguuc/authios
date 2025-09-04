impl crate::GroupsUseCase {
    /// # GroupsUseCase::revoke
    ///
    /// revoke a group from a group, checking for possible errors
    ///
    /// Errors:
    /// + when the group with provided name do not exist;
    /// + when the user with provided name do not exist;
    /// + when the user with provided login didn't had provided group;
    /// + when database connection cannot be acquired;
    ///
    pub async fn revoke<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, token: &String, encoding_key: &String, client: A) -> Result<(), GroupRevokeError> {
        type Error = GroupRevokeError;

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
        
        let user = crate::UsersRepository::retrieve(user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        // not added yet
        if !user.groups.contains(group_name) {
            return Err(Error::NotAddedYet);
        }
        
        // this won't error so we can skip this result
        let _ = crate::UserGroupsRepository::delete(user_login, group_name, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupRevokeError {
    #[error("GROUP_NOT_EXIST")]
    GroupNotExist,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
