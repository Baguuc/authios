use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::revoke
    ///
    /// Revoke a group from a group, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::revoke_group::UserRevokeGroupParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::revoke_group::UserRevokeGroupError]
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
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::UserRevokeGroupError as Error;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParams as Params;

            let params = Params {
                permission_name: String::from("authios:all"),
                encryption_key: params.encryption_key,
                token: params.token
            };
            
            match UsersUseCase::authorize(params, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }
        
        // check if the group exists
        {
            use crate::params::repository::GroupRetrieveParams as Params;

            let _ = GroupsRepository::retrieve(Params { name: params.group_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::GroupNotFound)?;
        }

        // retrieve the user
        let user = {    
            use crate::params::repository::UserRetrieveParams as Params;

            UsersRepository::retrieve(Params { login: params.user_login.clone() }, &mut *client)
                .await
                .map_err(|_| Error::UserNotFound)?
        };
        
        // not added yet
        if !user.groups.contains(&params.group_name.clone()) {
            return Err(Error::NotAddedYet);
        }
        
        // delete the group entry
        {
            use crate::params::repository::UserGroupDeleteParams as Params;

            let _ = UserGroupsRepository::delete(Params { user_login: params.user_login, group_name: params.group_name }, &mut *client)
                .await;
        }
        
        return Ok(());
    }
}
