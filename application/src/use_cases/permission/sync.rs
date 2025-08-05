impl crate::PermissionsUseCase {
    /// # PermissionsUseCase::sync
    ///
    /// sync permissions from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when one of permissions cannot be created;
    /// + when one of permissions cannot be delete (if necessary);
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(new: Vec<String>, client: A) -> Result<(), PermissionSyncError> {
        type Error = PermissionSyncError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let old = Self::list(&mut *client)
            .await
            .map_err(|_| Error::Generic)?
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);

        for permission_name in changes.delete {
            let _ = Self::delete(&permission_name, &mut *client)
                .await?;
        }

        for permission_name in changes.create {
            let permission = authios_domain::Permission {
                name: permission_name
            };

            let _ = Self::create(&permission, &mut *client)
                .await?;
        }
        
        return Ok(());
    }
}

pub enum PermissionSyncError {
    Generic,
    PermissionCreate(crate::use_cases::permission::create::PermissionCreateError),
    PermissionDelete(crate::use_cases::permission::delete::PermissionDeleteError)
}

impl ToString for PermissionSyncError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC"),
            Self::PermissionCreate(error) => error.to_string(),
            Self::PermissionDelete(error) => error.to_string()
        };
    }
}

impl From<crate::use_cases::permission::create::PermissionCreateError> for PermissionSyncError {
    fn from(error: crate::use_cases::permission::create::PermissionCreateError) -> Self {
        return Self::PermissionCreate(error);
    }
}

impl From<crate::use_cases::permission::delete::PermissionDeleteError> for PermissionSyncError {
    fn from(error: crate::use_cases::permission::delete::PermissionDeleteError) -> Self {
        return Self::PermissionDelete(error);
    }
}
