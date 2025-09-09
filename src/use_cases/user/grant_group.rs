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
        params: crate::params::use_case::UserGrantGroupParams,
        client: A
    ) -> Result<(), crate::errors::use_case::UserGrantGroupError> {
        use crate::repositories::{
            GroupsRepository,
            UsersRepository,
            UserGroupsRepository
        };
        use crate::errors::use_case::UserGrantGroupError as Error;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // check if group exists
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
        
        // check if permission exists
        {
            use crate::params::repository::UserRetrieveParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_login(params.user_login.clone())
                .build()
                .unwrap();

            let _ = UsersRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::UserNotExist)?;
        }
        
        // insert the data
        {
            use crate::params::repository::UserGroupInsertParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_group_name(params.group_name)
                .set_user_login(params.user_login)
                .build()
                .unwrap();
            
            UserGroupsRepository::insert(params, &mut *client)
                .await
                // already added
                .map_err(|_| Error::AlreadyAdded)?;
        }
        
        return Ok(());
    }
}
