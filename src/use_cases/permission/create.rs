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
        params: crate::params::use_case::PermissionCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::PermissionCreateError> {
        use crate::repositories::PermissionsRepository; 
        use crate::errors::use_case::PermissionCreateError as Error;
        use crate::params::repository::PermissionInsertParamsBuilder as ParamsBuilder;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let params = ParamsBuilder::new()
            .set_name(params.name)
            .build()
            .unwrap();
                
        PermissionsRepository::insert(params, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}
