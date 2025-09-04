impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name already exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<(), PermissionCreateError> {
        type Error = PermissionCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        crate::PermissionsRepository::insert(name, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
}
