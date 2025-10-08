pub struct UserUseCase;

impl UserUseCase {
    /// ### Description
    /// retrieve user data from JWT session token
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserInfoParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a fetched user or error of type
    /// [crate::errors::use_case::UserInfoError] inside.
    ///
    pub async fn info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserInfoParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::UserInfoError> {
        use crate::utils::jwt_token::get_claims;
        use crate::errors::use_case::UserInfoError as Error;
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
    /// log in as a user returning session token
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserLoginParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a JWT session token or error of type
    /// [crate::errors::use_case::UserLoginError] inside.
    /// 
    pub async fn login<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserLoginParams<'a>,
        database_client: A
    ) -> Result<String, crate::errors::use_case::UserLoginError> {
        use crate::utils::{
            password_hash::verify_password,
            jwt_token::generate as generate_token
        };
        use crate::repositories::UserRepository;
        use crate::params::repository::UserRetrieveByLoginParams;
        use crate::errors::use_case::UserLoginError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        let user = UserRepository::retrieve_by_login(
            UserRetrieveByLoginParams { login: params.login },
            &mut *database_client
        )
            .await
            .ok_or(Error::NotFound)?;

        if !verify_password(params.password, &user.password_hash) {
            return Err(Error::WrongPassword);
        }

        let token = generate_token(user.id, params.jwt_encryption_key).unwrap();

        Ok(token)
    }
    
    /// ### Description
    /// register a user
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserRegisterParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a fetched user or error of type
    /// [crate::errors::use_case::UserRegisterError] inside.
    ///
    pub async fn register<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserRegisterParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::UserRegisterError> {
        use crate::utils::password_hash::hash_password;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserInsertParams;
        use crate::errors::use_case::UserRegisterError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        let password_hash = hash_password(params.password).unwrap();
        
        UserRepository::insert(UserInsertParams { login: params.login, password_hash: &password_hash }, &mut *database_client)
            .await
            .map(|_| ())
            .map_err(|_| Error::AlreadyExists)
    }
    
    /// ### Description
    /// delete user from currently logged in 
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserDeleteAsSelfParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a unit type value ("()") or error of type
    /// [crate::errors::use_case::UserDeleteAsSelfError] inside.
    /// 
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserDeleteParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::UserDeleteError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserDeleteParams;
        use crate::errors::use_case::UserDeleteError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        // it is safe as to have a JWT token the user had to login first so we know he is
        // authorized to delete his account
        let claims = get_claims(params.token, params.jwt_encryption_key).unwrap();
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
    /// 1. params: [crate::params::use_case::UserUpdateParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the updated user data or error of type
    /// [crate::errors::use_case::UserUpdateError] inside.
    /// 
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserUpdateParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::UserUpdateError> {
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
        use crate::errors::use_case::UserUpdateError as Error;
        
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
    /// 1. params: [crate::params::use_case::UserListResourcePermissionsParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either the list of fetched permissions or error of type
    /// [crate::errors::use_case::UserListResourcePermissionsError] inside.
    /// 
    pub async fn list_resource_permissions<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserListResourcePermissionsParams<'a>,
        database_client: A
    ) -> Result<crate::models::UserResourcePermissionPage, crate::errors::use_case::UserListResourcePermissionsError> {
        use crate::utils::jwt_token::get_claims;
        use crate::models::UserResourcePermissionPage;
        use crate::repositories::UserResourcePermissionRepository;
        use crate::params::repository::{
            UserResourcePermissionListParams as ListParams,
            UserResourcePermissionGetPageCountParams as GetCountParams
        };
        use crate::errors::use_case::UserListResourcePermissionsError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        
        let claims = get_claims(&params.token, &params.jwt_encryption_key).map_err(|_| Error::InvalidToken)?;
        let user_id = claims.sub;

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
    /// 1. params: [crate::params::use_case::UserCheckResourcePermissionParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a boolean indicating if user has specified permission or error
    /// of type
    /// [crate::errors::use_case::UserCheckResourcePermissionError] inside.
    /// 
    pub async fn check_resource_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserCheckResourcePermissionParams<'a>,
        database_client: A
    ) -> Result<bool, crate::errors::use_case::UserCheckResourcePermissionError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::{
            ResourcePermissionRepository,
            UserResourcePermissionRepository
        };
        use crate::params::repository::{
            ResourcePermissionRetrieveParams,
            UserResourcePermissionRetrieveParams
        };
        use crate::errors::use_case::UserCheckResourcePermissionError as Error;

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
}
