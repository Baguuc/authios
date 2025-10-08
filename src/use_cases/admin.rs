pub struct AdminUseCase;

impl AdminUseCase {
    /// ### Description
    /// list user's resource permissions attached to specified service and resource type.
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AdminListUserResourcePermissionsParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the list of fetched permissions or error of type
    /// [crate::errors::use_case::AdminListUserResourcePermissionsError] inside.
    /// 
    pub async fn list_user_resource_permissions<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminListUserResourcePermissionsParams<'a>,
        database_client: A
    ) -> Result<crate::models::UserResourcePermissionPage, crate::errors::use_case::AdminListUserResourcePermissionsError> {
        use crate::models::UserResourcePermissionPage;
        use crate::repositories::UserResourcePermissionRepository;
        use crate::params::repository::{
            UserResourcePermissionListParams as ListParams,
            UserResourcePermissionGetPageCountParams as GetCountParams
        };
        use crate::errors::use_case::AdminListUserResourcePermissionsError as Error;

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let permissions = UserResourcePermissionRepository::list(
            ListParams { user_id: params.id, service_id: params.service_id, resource_type: params.resource_type, page_number: &Some(params.page_number.clone()) },
            &mut *database_client
        ).await;

        let total_page_count = UserResourcePermissionRepository::get_page_count(
            GetCountParams { user_id: params.id, service_id: params.service_id, resource_type: params.resource_type },
            &mut *database_client
        ).await;

        let page = UserResourcePermissionPage {
            page_number: params.page_number.clone(),
            total_page_count,
            permissions
        };
        
        Ok(page)
    }

    /// ### Description
    /// get user info as admin 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AdminGetUserInfoParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a unit type value ("()") or error of type
    /// [crate::errors::use_case::AdminGetUserInfoError] inside.
    /// 
    pub async fn get_user_info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminGetUserInfoParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::AdminGetUserInfoError> {
        use crate::repositories::UserRepository;
        use crate::params::repository::UserRetrieveParams;
        use crate::errors::use_case::AdminGetUserInfoError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        let user = UserRepository::retrieve(
            UserRetrieveParams { id: params.id },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotFound)?;
        

        Ok(user)
    }

    /// ### Description
    /// Grant user with specified id a resource permission matching specified criteria authorizing
    /// the operation with root password
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AdminGrantUserResourcePermissionParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with error of type [crate::errors::use_case::AdminGrantUserResourcePermissionError] inside.
    /// 
    pub async fn grant_resource_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminGrantUserResourcePermissionParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::AdminGrantUserResourcePermissionError> {
        use crate::repositories::{
            UserResourcePermissionRepository,
            UserRepository,
            ResourcePermissionRepository
        };
        use crate::params::repository::{
            ResourcePermissionRetrieveParams,
            UserRetrieveParams,
            UserResourcePermissionInsertParams
        };
        use crate::errors::use_case::AdminGrantUserResourcePermissionError as Error;
        
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        let _ = UserRepository::retrieve(
            UserRetrieveParams {
                id: params.user_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::UserNotFound)?;


        let resource_permission = ResourcePermissionRepository::retrieve(
            ResourcePermissionRetrieveParams { 
                service_id: params.service_id,
                resource_type: params.resource_type,
                permission_name: params.permission_name
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::PermissionNotFound)?;

        let _ = UserResourcePermissionRepository::insert(
            UserResourcePermissionInsertParams {
                resource_permission_id: &resource_permission.id,
                user_id: params.user_id,
                resource_id: params.resource_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::AlreadyAdded)?;

        Ok(())
    }
    
    /// ### Description
    /// Revoke user with specified id a resource permission matching specified criteria authorizing
    /// the operation with root password
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AdminRevokeUserResourcePermissionParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with error of type
    /// [crate::errors::use_case::AdminRevokeUserResourcePermissionError] inside.
    /// 
    pub async fn revoke_resource_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminRevokeUserResourcePermissionParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::AdminRevokeUserResourcePermissionError> {
        use crate::repositories::{
            UserResourcePermissionRepository,
            UserRepository,
            ResourcePermissionRepository
        };
        use crate::params::repository::{
            ResourcePermissionRetrieveParams,
            UserRetrieveParams,
            UserResourcePermissionDeleteParams
        };
        use crate::errors::use_case::AdminRevokeUserResourcePermissionError as Error;
        
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        let _ = UserRepository::retrieve(
            UserRetrieveParams {
                id: params.user_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::UserNotFound)?;

        let resource_permission = ResourcePermissionRepository::retrieve(
            ResourcePermissionRetrieveParams { 
                service_id: params.service_id,
                resource_type: params.resource_type,
                permission_name: params.permission_name
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::PermissionNotFound)?;

        let _ = UserResourcePermissionRepository::delete(
            UserResourcePermissionDeleteParams {
                resource_permission_id: &resource_permission.id,
                user_id: params.user_id,
                resource_id: params.resource_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotAddedYet)?;

        Ok(())
    }

    /// ### Description
    /// delete a user as admin 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserDeleteAsAdminParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a unit type value ("()") or error of type
    /// [crate::errors::use_case::UserDeleteAsAdminError] inside.
    /// 
    pub async fn delete_user<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminDeleteUserParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::AdminDeleteUserError> {
        use crate::repositories::UserRepository;
        use crate::params::repository::UserDeleteParams;
        use crate::errors::use_case::AdminDeleteUserError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        UserRepository::delete(
            UserDeleteParams { id: params.id },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotFound)?;
        

        Ok(())
    }
    
    /// ### Description
    /// update user's data as admin
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AdminUpdateUserParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the updated user data or error of type
    /// [crate::errors::use_case::AdminUpdateUserError] inside.
    /// 
    pub async fn update_user<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AdminUpdateUserParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::AdminUpdateUserError> {
        use crate::utils::password_hash::hash_password;
        use crate::repositories::UserRepository;
        use crate::params::repository::{
            UserUpdateParams,
            UserRetrieveParams
        };
        use crate::models::User;
        use crate::errors::use_case::AdminUpdateUserError as Error;

        if params.password != params.root_password {
            return Err(Error::Unauthorized);
        }
        
        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        let original_data = UserRepository::retrieve(UserRetrieveParams { id: &params.id }, &mut *database_client)
            .await
            .ok_or(Error::NotFound)?;

        let new_login = if let Some(login) = params.new_login 
            { login } else { &original_data.login };

        let new_password_hash = if let Some(password) = params.new_password
            { hash_password(&password).unwrap() } else { original_data.password_hash };

        UserRepository::update(UserUpdateParams { id: params.id, login: &new_login, password_hash: &new_password_hash }, &mut *database_client)
            .await
            .ok_or(Error::NotFound)?;

        let new_data = User {
            id: original_data.id,
            login: new_login.clone(),
            password_hash: new_password_hash
        };

        Ok(new_data)
    }
}
