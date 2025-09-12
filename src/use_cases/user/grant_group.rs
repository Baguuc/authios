use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::grant_group
    ///
    /// Grant a group to a user, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::grant_group::UserGrantGroupParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::grant_group::UserGrantGroupError]
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
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::UserGrantGroupError as Error;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_token(params.token)
                .set_encryption_key(params.encryption_key)
                .set_permission_name(String::from("authios:all"))
                .build()
                .unwrap();
            
            match UsersUseCase::authorize(params, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }
        
        // check if group exists
        {
            use crate::params::repository::GroupRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.group_name.clone())
                .build()
                .unwrap();

            let _ = GroupsRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::GroupNotFound)?;
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
                .map_err(|_| Error::UserNotFound)?;
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
