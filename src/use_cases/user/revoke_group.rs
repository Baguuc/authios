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
        params: crate::params::use_case::UserRevokeGroupParams,
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
        
        // check if the group exists
        {
            use crate::params::repository::GroupRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.group_name.clone())
                .build()
                .unwrap();

            let _ = GroupsRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::GroupNotExist)?;
        }

        // retrieve the user
        let user = {    
            use crate::params::repository::UserRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_login(params.user_login.clone())
                .build()
                .unwrap();

            UsersRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::UserNotExist)?
        };
        
        // not added yet
        if !user.groups.contains(&params.group_name.clone()) {
            return Err(Error::NotAddedYet);
        }
        
        // delete the group entry
        {
            use crate::params::repository::UserGroupDeleteParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_user_login(params.user_login)
                .set_group_name(params.group_name)
                .build()
                .unwrap();

            let _ = UserGroupsRepository::delete(params, &mut *client)
                .await;
        }
        
        return Ok(());
    }
}
