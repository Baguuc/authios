impl crate::UsersUseCase {
    /// # UsersUseCase::check_permission
    ///
    /// check if user has patricular permission
    ///
    /// Errors:
    /// + when provided token is invalid;
    /// + when a user with token login do not exist;
    /// + when a permission with provided name do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn check_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(token: &String, encoding_key: &String, permission_name: &String, client: A) -> Result<bool, UserCheckPermissionError> {
        use crate::use_cases::user::retrieve_from_token::UserRetrieveFromTokenError;
        
        type Error = UserCheckPermissionError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // will optimize all of this if necessary
        let _ = crate::PermissionsRepository::retrieve(permission_name, &mut *client)
            .await
            .map_err(|_| Error::PermissionNotExist)?;

        let user = Self::retrieve_from_token(token, encoding_key, &mut *client)
            .await
            .map_err(|error| match error {
                 UserRetrieveFromTokenError::InvalidToken => Error::InvalidToken,
                 UserRetrieveFromTokenError::NotExist => Error::UserNotExist,
                 UserRetrieveFromTokenError::DatabaseConnection => Error::DatabaseConnection,
            })?;
        
        let mut permissions = vec![];

        for group_name in user.groups {
            let group = crate::GroupsRepository::retrieve(&group_name, &mut *client)
                .await
                // this won't error as we just fetched the permissions
                .unwrap();

            permissions.extend(group.permissions);
        }

        let permissions = permissions;
        
        return Ok(permissions.contains(permission_name));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserCheckPermissionError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("USER_NOT_EXIST")]
    UserNotExist,
    #[error("PERMISSION_NOT_EXIST")]
    PermissionNotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
