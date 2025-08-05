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
    pub async fn revoke<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, client: A) -> Result<(), GroupRevokeError> {
        type Error = GroupRevokeError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let user = crate::UsersRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        // not added yet
        if !user.groups.contains(group_name) {
            return Err(Error::Generic);
        }
        
        // this won't error so we can skip this result
        let _ = crate::UserGroupsRepository::delete(user_login, group_name, &mut *client)
            .await;
        
        return Ok(());
    }
}

pub enum GroupRevokeError {
    Generic
}

impl ToString for GroupRevokeError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
