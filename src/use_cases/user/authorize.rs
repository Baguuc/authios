use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::authorize
    ///
    /// check if user has patricular permission
    ///
    /// Errors:
    /// + when provided token is invalid;
    /// + when a user with token login do not exist;
    /// + when a permission with provided name do not exist;
    /// + when database connection cannot be acquired;
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
            use crate::params::repository::PermissionRetrieveParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_name(params.permission_name.clone())
                .build()
                .unwrap();

            let _ = PermissionsRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::PermissionNotFound)?;
        }


        // retrieve the user
        let user = {
            use crate::params::repository::UserRetrieveParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_login(user_login)
                .build()
                .unwrap();
            
            // invalid token points to non-existent user
            let user = UsersRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::InvalidToken)?;

            user
        };


        for group_name in user.groups {
            use crate::params::repository::GroupRetrieveParamsBuilder as ParamsBuilder;

            let rparams = ParamsBuilder::new()
                .set_name(group_name)
                .build()
                .unwrap();

            let group = GroupsRepository::retrieve(rparams, &mut *client)
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
