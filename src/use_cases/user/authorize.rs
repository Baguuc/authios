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
        params: crate::params::UserAuthorizeParams,
        client: A
    ) -> Result<bool, crate::errors::use_case::UserAuthorizeError> {
        use crate::repositories::{
            PermissionsRepository,
            GroupsRepository,
            UsersRepository
        };
        use crate::errors::use_case::UserAuthorizeError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // will optimize all of this if necessary
        let _ = PermissionsRepository::retrieve(&params.permission_name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;


        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;

        let user = UsersRepository::retrieve(&claims.sub, &mut *client)
            .await
            .map_err(|_| Error::UserNotExist)?;
        

        let mut permissions = vec![];

        for group_name in user.groups {
            let group = GroupsRepository::retrieve(&group_name, &mut *client)
                .await
                // this won't error as we just fetched the permissions
                .unwrap();

            permissions.extend(group.permissions);
        }

        let permissions = permissions;
        
        return Ok(permissions.contains(&params.permission_name));
    }
}
