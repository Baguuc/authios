use crate::use_cases::PermissionsUseCase;

impl PermissionsUseCase {
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
        params: crate::params::PermissionCreateParams,
        client: A
    ) -> Result<(), crate::errors::PermissionCreateError> {
        use crate::repositories::PermissionsRepository; 
        use crate::errors::PermissionCreateError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
                
        PermissionsRepository::insert(&params.name, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}
