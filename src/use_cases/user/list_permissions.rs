use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::list_permissions
    ///
    /// List all user's permissions
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::list_permissions::UserListPermissionsParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::list_permissions::UserListPermissionsError]
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
            use crate::params::repository::UserRetrieveParams as Params;

            UsersRepository::retrieve(Params { login: user_login }, &mut *client)
                .await
                .map_err(|_| Error::InvalidToken)?
        };

        let mut permissions = HashSet::new();
        
        // fetch all the groups and extend the permissions set
        {
            use crate::params::repository::GroupRetrieveParams as Params;
            
            for group_name in user.groups {
                let group = GroupsRepository::retrieve(Params { name: group_name }, &mut *client)
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
