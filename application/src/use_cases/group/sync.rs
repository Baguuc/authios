impl crate::GroupsUseCase {
    /// # GroupsUseCase::sync
    ///
    /// sync groups from config with database, checking for possible errors
    ///
    /// Errors:
    /// + when one of the groups cannot be created;
    /// + when one of the groups cannot be delete (if necessary);
    /// + when one of the group's permission cannot be granted;
    /// + when database connection cannot be acquired;
    ///
    pub async fn sync<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(new: Vec<authios_domain::Group>, client: C) -> Result<(), GroupSyncError> {        
        type Error = GroupSyncError; 

        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let old = Self::list(&mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let changes = crate::utils::detect_changes_in_vecs(old, new);
        
        for group in changes.delete {
            let _ = Self::delete(&group.name, &mut *client)
                .await?;
        }

        for group in changes.create {
            let _ = Self::create(&group, &mut *client)
                .await?;

            for permission in group.permissions {
                 let _ = crate::PermissionsUseCase::grant(&permission, &group.name, &mut *client).await?;
            }
        }
        
        return Ok(());
    }
}

pub enum GroupSyncError {
    Generic,
    GroupCreate(crate::use_cases::group::create::GroupCreateError),
    GroupDelete(crate::use_cases::group::delete::GroupDeleteError),
    PermissionGrant(crate::use_cases::permission::grant::PermissionGrantError)
}

impl ToString for GroupSyncError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC"),
            Self::GroupCreate(error) => error.to_string(),
            Self::GroupDelete(error) => error.to_string(),
            Self::PermissionGrant(error) => error.to_string(),
        };
    }
}

impl From<crate::use_cases::group::create::GroupCreateError> for GroupSyncError {
    fn from(error: crate::use_cases::group::create::GroupCreateError) -> Self {
        return Self::GroupCreate(error);
    }
}

impl From<crate::use_cases::group::delete::GroupDeleteError> for GroupSyncError {
    fn from(error: crate::use_cases::group::delete::GroupDeleteError) -> Self {
        return Self::GroupDelete(error);
    }
}

impl From<crate::use_cases::permission::grant::PermissionGrantError> for GroupSyncError {
    fn from(error: crate::use_cases::permission::grant::PermissionGrantError) -> Self {
        return Self::PermissionGrant(error);
    }
}
