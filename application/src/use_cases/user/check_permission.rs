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
        type Error = UserCheckPermissionError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        // will optimize all of this if necessary
        let _ = crate::PermissionsRepository::retrieve(permission_name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;

        let user = crate::UsersUseCase::retrieve_from_token(token, encoding_key, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let data = crate::UsersRepository::retrieve(&user.login, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;

        let mut permissions = vec![];

        for group_name in data.groups {
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

pub enum UserCheckPermissionError {
    Generic
}

impl ToString for UserCheckPermissionError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
