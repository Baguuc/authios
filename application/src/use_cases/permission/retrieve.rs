impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::retrieve
    ///
    /// retrieve a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<authios_domain::Permission, PermissionRetrieveError> {
        type Error = PermissionRetrieveError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let data = crate::PermissionsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?; 
        
        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionRetrieveError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
