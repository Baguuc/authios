use crate::use_cases::UsersUseCase;

impl UsersUseCase {
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
        params: crate::params::UserGrantGroupParams,
        client: A
    ) -> Result<(), crate::errors::UserGrantGroupError> {
        use crate::repositories::{
            GroupsRepository,
            UsersRepository,
            UserGroupsRepository
        };
        use crate::errors::UserGrantGroupError as Error;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        let _ = UsersRepository::retrieve(&params.user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        UserGroupsRepository::insert(&params.user_login, &params.group_name, &mut *client)
            .await
            // already added
            .map_err(|_| Error::AlreadyAdded)?;
        
        return Ok(());
    }
}
