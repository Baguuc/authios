pub struct LoggedUserUseCase;

impl LoggedUserUseCase {
    /// ### Description
    /// retrieve user data from JWT session token
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserInfoParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a fetched user or error of type
    /// [crate::errors::use_case::LoggedUserInfoError] inside.
    ///
    pub async fn info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserInfoParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::LoggedUserInfoError> {
        use crate::utils::jwt_token::get_claims;
        use crate::errors::use_case::LoggedUserInfoError as Error;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserRetrieveParams;
        
        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        let claims = get_claims(params.token, params.jwt_encryption_key).map_err(|_| Error::InvalidToken)?;
        
        UserRepository::retrieve(UserRetrieveParams { id: &claims.sub }, &mut *database_client)
            .await
            .ok_or(Error::InvalidToken)
    }
    
    /// ### Description
    /// delete user from currently logged in 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserDeleteAsSelfParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a unit type value ("()") or error of type
    /// [crate::errors::use_case::LoggedUserDeleteAsSelfError] inside.
    /// 
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserDeleteParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::LoggedUserDeleteError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserDeleteParams;
        use crate::errors::use_case::LoggedUserDeleteError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        // it is safe as to have a JWT token the user had to login first so we know he is
        // authorized to delete his account
        let claims = get_claims(params.token, params.jwt_encryption_key).map_err(|_| Error::InvalidToken)?;
        let user_id = claims.sub;
        
        UserRepository::delete(
            UserDeleteParams { id: &user_id },
            &mut *database_client
        )
            .await
            .ok_or(Error::InvalidToken)?;
        

        Ok(())
    }

    /// ### Description
    /// update user's data
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserUpdateParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the updated user data or error of type
    /// [crate::errors::use_case::LoggedUserUpdateError] inside.
    /// 
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserUpdateParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::LoggedUserUpdateError> {
        use crate::utils::{
            jwt_token::get_claims,
            password_hash::hash_password
        };
        use crate::repositories::UserRepository;
        use crate::params::repository::{
            UserUpdateParams,
            UserRetrieveParams
        };
        use crate::models::User;
        use crate::errors::use_case::LoggedUserUpdateError as Error;
        
        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        // it is safe as to have a JWT token the user had to login first so we know he is
        // authorized to update his own account
        let claims = get_claims(params.token, params.jwt_encryption_key).unwrap();
        let user_id = claims.sub;

        let original_data = UserRepository::retrieve(UserRetrieveParams { id: &user_id }, &mut *database_client)
            .await
            .ok_or(Error::InvalidToken)?;

        let new_login = if let Some(login) = params.new_login 
            { login } else { &original_data.login };

        let new_password_hash = if let Some(password) = params.new_password
            { hash_password(&password).unwrap() } else { original_data.password_hash };

        UserRepository::update(UserUpdateParams { id: &user_id, login: &new_login, password_hash: &new_password_hash }, &mut *database_client)
            .await
            .ok_or(Error::InvalidToken)?;

        let new_data = User {
            id: original_data.id,
            login: new_login.clone(),
            password_hash: new_password_hash
        };

        Ok(new_data)
    }

     /// ### Description
    /// list user's resource permissions attached to specified service and resource type.
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserListResourcePermissionsParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the list of fetched permissions or error of type
    /// [crate::errors::use_case::LoggedUserListResourcePermissionsError] inside.
    /// 
    pub async fn list_resource_permissions<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserListResourcePermissionsParams<'a>,
        database_client: A
    ) -> Result<crate::models::UserResourcePermissionPage, crate::errors::use_case::LoggedUserListResourcePermissionsError> {
        use crate::utils::jwt_token::get_claims;
        use crate::models::UserResourcePermissionPage;
        use crate::repositories::{
            UserResourcePermissionRepository,
            UserRepository
        };
        use crate::params::repository::{
            UserResourcePermissionListParams as ListParams,
            UserResourcePermissionGetPageCountParams as GetCountParams,
            UserRetrieveParams
        };
        use crate::errors::use_case::LoggedUserListResourcePermissionsError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        let claims = get_claims(&params.token, &params.jwt_encryption_key).map_err(|_| Error::InvalidToken)?;
        let user_id = claims.sub;
        
        // check if user exist
        let _ = UserRepository::retrieve(
            UserRetrieveParams {
                id: &user_id
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::InvalidToken)?;

        let permissions = UserResourcePermissionRepository::list(
            ListParams { user_id: &user_id, service_id: params.service_id, resource_type: params.resource_type, page_number: &Some(params.page_number.clone()) },
            &mut *database_client
        ).await;

        let total_page_count = UserResourcePermissionRepository::get_page_count(
            GetCountParams { user_id: &user_id, service_id: params.service_id, resource_type: params.resource_type },
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
    /// check if user is permitted to specified resource with specifed permission.
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserCheckResourcePermissionParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a boolean indicating if user has specified permission or error
    /// of type
    /// [crate::errors::use_case::LoggedUserCheckResourcePermissionError] inside.
    /// 
    pub async fn check_resource_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserCheckResourcePermissionParams<'a>,
        database_client: A
    ) -> Result<bool, crate::errors::use_case::LoggedUserCheckResourcePermissionError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::{
            ResourcePermissionRepository,
            UserResourcePermissionRepository
        };
        use crate::params::repository::{
            ResourcePermissionRetrieveParams,
            UserResourcePermissionRetrieveParams
        };
        use crate::errors::use_case::LoggedUserCheckResourcePermissionError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        let claims = get_claims(&params.token, &params.jwt_encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_id = claims.sub;
        
        // check if permission exist
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

        match UserResourcePermissionRepository::retrieve(
            UserResourcePermissionRetrieveParams { 
                user_id: &user_id, 
                permission_id: &resource_permission.id,
                resource_id: params.resource_id
            },
            &mut *database_client
        ).await {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }
    
    /// ### Description
    /// check if user is permitted to specified service with specifed permission.
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::LoggedUserCheckServicePermissionParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a boolean indicating if user has specified permission or error
    /// of type
    /// [crate::errors::use_case::LoggedUserCheckServicePermissionError] inside.
    /// 
    pub async fn check_service_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::LoggedUserCheckServicePermissionParams<'a>,
        database_client: A
    ) -> Result<bool, crate::errors::use_case::LoggedUserCheckServicePermissionError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::{
            ServicePermissionRepository,
            UserServicePermissionRepository
        };
        use crate::params::repository::{
            ServicePermissionRetrieveParams,
            UserServicePermissionRetrieveParams
        };
        use crate::errors::use_case::LoggedUserCheckServicePermissionError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        let claims = get_claims(&params.token, &params.jwt_encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_id = claims.sub;
        
        // check if permission exist
        let service_permission = ServicePermissionRepository::retrieve(
            ServicePermissionRetrieveParams { 
                service_id: params.service_id 
            },
            &mut *database_client
        )
            .await
            .ok_or(Error::PermissionNotFound)?;

        match UserServicePermissionRepository::retrieve(
            UserServicePermissionRetrieveParams { 
                user_id: &user_id, 
                permission_id: &service_permission.id
            },
            &mut *database_client
        ).await {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }
}
