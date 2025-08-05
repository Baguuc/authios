impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::delete
    ///
    /// delete a permission, checking for possible errors
    ///
    /// Errors:
    /// + when a permission with provided name do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<(), PermissionDeleteError> {
        type Error = PermissionDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::PermissionsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        // this won't error so we can skip this result
        let _ = crate::PermissionsRepository::delete(name, &mut *client)
            .await;
        
        return Ok(());
    }
}

pub enum PermissionDeleteError {
    Generic
}

impl ToString for PermissionDeleteError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
