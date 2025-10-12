pub struct ServicePermissionUseCase;

impl ServicePermissionUseCase {
    /// ### Description
    /// create a service permission authorized by root password from config 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::ServicePermissionCreateParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with a error of type
    /// [crate::errors::use_case::ServicePermissionCreateError] inside.
    /// 
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::ServicePermissionCreateParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::ServicePermissionCreateError> {
        use crate::repositories::ServicePermissionRepository;
        use crate::params::repository::ServicePermissionInsertParams as Params;
        use crate::errors::use_case::ServicePermissionCreateError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        ServicePermissionRepository::insert(
            Params {
                service_id: &params.service_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::AlreadyExists)?;

        Ok(())
    }
    
    /// ### Description
    /// delete a service permission authorized by root password from config 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::ServicePermissionDeleteParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with a error of type
    /// [crate::errors::use_case::ServicePermissionDeleteError] inside.
    /// 
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::ServicePermissionDeleteParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::ServicePermissionDeleteError> {
        use crate::repositories::ServicePermissionRepository;
        use crate::params::repository::ServicePermissionDeleteParams as Params;
        use crate::errors::use_case::ServicePermissionDeleteError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        ServicePermissionRepository::delete(
            Params {
                service_id: &params.service_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotFound)?;

        Ok(())
    }
}
