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
            .map_err(|_| Error::Generic)?;
        
        let data = crate::PermissionsRepository::list(&mut *client)
            .await
            .unwrap_or(vec![]);
        
        return Ok(data);
    }
}

pub enum PermissionListError {
    Generic
}

impl ToString for PermissionListError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
