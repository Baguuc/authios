use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::list_permissions
    ///
    /// list all user's permissions
    ///
    /// Errors:
    /// + when the provided token is invalid;
    /// + when a user login specified in the token not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn list_permissions<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserListPermissionsParams,
        client: A
    ) -> Result<Vec<String>, crate::errors::use_case::UserListPermissionsError> {
        use std::collections::HashSet; 
        use crate::repositories::{
            UsersRepository,
            GroupsRepository
        }; 
        use crate::errors::use_case::UserListPermissionsError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // get the user login from the token
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_login = claims.sub;

        // get the user's data
        let user = {
            use crate::params::repository::UserRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_login(user_login)
                .build()
                .unwrap();

            UsersRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::UserNotExist)?
        };

        let mut permissions = HashSet::new();
        
        // fetch all the groups and extend the permissions set
        {
            use crate::params::repository::GroupRetrieveParamsBuilder as ParamsBuilder;
            
            for group_name in user.groups {
                let params = ParamsBuilder::new()
                    .set_name(group_name)
                    .build()
                    .unwrap();

                let group = GroupsRepository::retrieve(params, &mut *client)
                    .await
                    .unwrap();

                permissions.extend(group.permissions);
            }
        };

        let permissions = permissions
            .into_iter()
            .collect::<Vec<String>>();
        
        return Ok(permissions);
    }
}
