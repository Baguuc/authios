impl crate::GroupsUseCase {
    /// # GroupsUseCase::delete
    ///
    /// delete a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name do not exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, token: &String, encoding_key: &String, client: A) -> Result<(), GroupDeleteError> {
        type Error = GroupDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        match crate::UsersUseCase::check_permission(token, encoding_key, &String::from("authios:root:write"), &mut *client).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::GroupsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        // this won't error so we can skip this result
        let _ = crate::GroupsRepository::delete(name, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupDeleteError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("UNAUTHORIZED_EXIST")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
