impl crate::GroupsUseCase {
    /// # GroupsUseCase::grant
    ///
    /// grant a group to a user, checking for possible errors
    ///
    /// Errors:
    /// + when the group with provided name do not exist;
    /// + when the user with provided name do not exist;
    /// + when the user with provided login already has provided group;
    /// + when database connection cannot be acquired;
    ///
    pub async fn grant<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, group_name: &String, client: A) -> Result<(), GroupGrantError> {
        type Error = GroupGrantError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::GroupsRepository::retrieve(group_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        crate::UserGroupsRepository::insert(user_login, group_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::Generic)?;
        
        return Ok(());
    }
}

pub enum GroupGrantError {
    Generic
}

impl ToString for GroupGrantError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
