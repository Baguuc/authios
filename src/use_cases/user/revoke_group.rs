use crate::use_cases::UsersUseCase;

impl UsersUseCase {
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
        params: crate::params::UserRevokeGroupParams,
        client: A
    ) -> Result<(), crate::errors::use_case::UserRevokeGroupError> {
        use crate::repositories::{
            GroupsRepository,
            UsersRepository,
            UserGroupsRepository
        };
        use crate::errors::use_case::UserRevokeGroupError as Error;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = GroupsRepository::retrieve(&params.group_name, &mut *client)
            .await
            .map_err(|_| Error::GroupNotExist)?;
        
        let user = UsersRepository::retrieve(&params.user_login, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        
        // not added yet
        if !user.groups.contains(&params.group_name) {
            return Err(Error::NotAddedYet);
        }
        
        // this won't error so we can skip this result
        let _ = UserGroupsRepository::delete(&params.user_login, &params.group_name, &mut *client)
            .await;
        
        return Ok(());
    }
}
