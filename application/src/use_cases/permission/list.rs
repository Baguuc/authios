impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::list
    ///
    /// list permissions
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(client: A) -> Result<Vec<authios_domain::Permission>, PermissionListError> {
        type Error = PermissionListError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let data = crate::PermissionsRepository::list(&mut *client)
            .await
            .unwrap_or(vec![]);
        
        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionListError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
