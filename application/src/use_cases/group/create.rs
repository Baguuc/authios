impl crate::GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// create a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name already exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        name: &String,
        token: &String,
        encoding_key: &String,
        client: A
    ) -> Result<(), GroupCreateError> {
        type Error = GroupCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        match crate::UsersUseCase::check_permission(token, encoding_key, &String::from("authios:root:write"), &mut *client).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        crate::GroupsRepository::insert(name, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GroupCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("UNAUTHORIZED_EXIST")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
