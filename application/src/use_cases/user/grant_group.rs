impl crate::UsersUseCase {
    /// # UsersUseCase::grant_group
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
    pub async fn grant_group<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::UserGrantGroupParams,
        client: A
    ) -> Result<(), authios_domain::UserGrantGroupError> {
        type Error = authios_domain::UserGrantGroupError;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        let _ = crate::UsersRepository::retrieve(&params.user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        crate::UserGroupsRepository::insert(&params.user_login, &params.group_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}
