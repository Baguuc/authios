pub struct ResourcePermissionUseCase;

impl ResourcePermissionUseCase {
    /// ### Description
    /// create a resource permission authorized by root password from config 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::ResourcePermissionCreateParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with a error of type
    /// [crate::errors::use_case::ResourcePermissionCreateError] inside.
    /// 
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::ResourcePermissionCreateParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::ResourcePermissionCreateError> {
        use crate::repositories::ResourcePermissionRepository;
        use crate::params::repository::ResourcePermissionInsertParams;
        use crate::errors::use_case::ResourcePermissionCreateError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        ResourcePermissionRepository::insert(
            ResourcePermissionInsertParams {
                service_id: &params.service_id,
                resource_type: &params.resource_type,
                permission_name: &params.permission_name
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::AlreadyExists)?;

        Ok(())
    }
    
    /// ### Description
    /// delete a resource permission authorized by root password from config 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::ResourcePermissionDeleteParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with a error of type
    /// [crate::errors::use_case::ResourcePermissionDeleteError] inside.
    /// 
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::ResourcePermissionDeleteParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::ResourcePermissionDeleteError> {
        use crate::repositories::ResourcePermissionRepository;
        use crate::params::repository::ResourcePermissionDeleteParams;
        use crate::errors::use_case::ResourcePermissionDeleteError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        ResourcePermissionRepository::delete(
            ResourcePermissionDeleteParams {
                service_id: &params.service_id,
                resource_type: &params.resource_type,
                permission_name: &params.permission_name
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotFound)?;

        Ok(())
    }
}
