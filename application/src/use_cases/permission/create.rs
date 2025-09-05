impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name already exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::PermissionCreateParams,
        client: A
    ) -> Result<(), PermissionCreateError> {
        type Error = PermissionCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
                
        crate::PermissionsRepository::insert(&params.name, &mut *client)
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
