pub struct UserUseCase;

impl UserUseCase {
    /// ### Description
    /// retrieve user data from JWT session token
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::UserRetrieveFromTokenParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a fetched user or error of type
    /// [crate::errors::use_case::UserRetrieveFromTokenError] inside.
    ///
    pub async fn retrieve_from_token<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserRetrieveFromTokenParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, crate::errors::use_case::UserRetrieveFromTokenError> {
        use crate::utils::jwt_token::get_claims;
        use crate::errors::use_case::UserRetrieveFromTokenError as Error;
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
            .map_err(|_| Error::AlreadyExist)
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
    pub async fn delete_as_admin<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserDeleteAsAdminParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::UserDeleteAsAdminError> {
        use crate::repositories::UserRepository;
        use crate::params::repository::UserDeleteParams;
        use crate::errors::use_case::UserDeleteAsAdminError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();

        if params.password != params.root_password {
            return Err(Error::WrongPassword);
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
    /// [crate::errors::use_case::UserAsSelfError] inside.
    /// 
    pub async fn delete_as_self<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserDeleteAsSelfParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::UserLoginError> {
        use crate::utils::jwt_token::get_claims;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserDeleteParams;
        use crate::errors::use_case::UserLoginError as Error;

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
            .ok_or(Error::NotFound)?;
        

        Ok(())
    }

    /// ### Description
    /// update user's data
    ///
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

        let new_login = params.new_login.unwrap_or(original_data.login);
        let new_password_hash = if let Some(password) = params.new_password {
            hash_password(&password).unwrap()
        } else { 
            original_data.password_hash
        };

        UserRepository::update(UserUpdateParams { id: &user_id, login: &new_login, password_hash: &new_password_hash }, &mut *database_client)
            .await
            .ok_or(Error::InvalidToken)?;

        let new_data = User {
            id: original_data.id,
            login: new_login,
            password_hash: new_password_hash
        };

        Ok(new_data)
    }
}
