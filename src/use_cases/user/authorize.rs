use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::authorize
    ///
    /// Check if user has patricular permission
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::authorize::UserAuthorizeParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::authorize::UserAuthorizeError]
    ///
    pub async fn authorize<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserAuthorizeParams,
        client: A
    ) -> Result<bool, crate::errors::use_case::UserAuthorizeError> {
        use crate::repositories::{
            PermissionsRepository,
            GroupsRepository,
            UsersRepository
        };
        use crate::errors::use_case::UserAuthorizeError as Error; 
        use crate::utils::jwt_token::get_claims;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let user_login = get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?
            .sub;
        
        // check if permission exist
        {
            use crate::params::repository::PermissionRetrieveParams as Params;

            let _ = PermissionsRepository::retrieve(Params { name: params.permission_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::PermissionNotFound)?;
        }


        // retrieve the user
        let user = {
            use crate::params::repository::UserRetrieveParams as Params;

            // invalid token points to non-existent user
            let user = UsersRepository::retrieve(Params { login: user_login }, &mut *client)
                .await
                .map_err(|_| Error::InvalidToken)?;

            user
        };


        for group_name in user.groups {
            use crate::params::repository::GroupRetrieveParams as Params;

            let group = GroupsRepository::retrieve(Params { name: group_name }, &mut *client)
                .await
                .unwrap();
            
            // user has the permission
            if group.permissions.contains(&params.permission_name) {
                return Ok(true);
            }
        }

        // user don't have the permission
        return Ok(false);
    }
}
