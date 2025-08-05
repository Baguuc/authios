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
            .map_err(|_| Error::Generic)?;
        
        let data = crate::PermissionsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(data);
    }
}

pub enum PermissionRetrieveError {
    Generic
}

impl ToString for PermissionRetrieveError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
