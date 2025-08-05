impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::create
    ///
    /// create a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name already exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(data: &authios_domain::Permission, client: A) -> Result<(), PermissionCreateError> {
        type Error = PermissionCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        crate::PermissionsRepository::insert(data, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(());
    }
}

pub enum PermissionCreateError {
    Generic
}

impl ToString for PermissionCreateError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
