impl crate::UsersUseCase {
    /// # UsersUseCase::revoke
    ///
    /// revoke a group from a group, checking for possible errors
    ///
    /// Errors:
    /// + when the group with provided name do not exist;
    /// + when the user with provided name do not exist;
    /// + when the user with provided login didn't had provided group;
    /// + when database connection cannot be acquired;
    ///
    pub async fn revoke_group<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::UserRevokeGroupParams,
        client: A
    ) -> Result<(), UserRevokeGroupError> {
        type Error = UserRevokeGroupError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        match crate::UsersUseCase::authorize(&params.auth.token, &params.auth.encoding_key, &String::from("authios:root:write"), &mut *client).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        let user = crate::UsersRepository::retrieve(&params.user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        // not added yet
        if !user.groups.contains(&params.group_name) {
            return Err(Error::NotAddedYet);
        }
        
        // this won't error so we can skip this result
        let _ = crate::UserGroupsRepository::delete(&params.user_login, &params.group_name, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserRevokeGroupError {
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
