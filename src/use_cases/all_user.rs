pub struct AllUserUseCase;

impl AllUserUseCase {
    /// ### Description
    /// log in as a user returning session token
    ///
    /// ### Arguments
    /// 1. params: [crate::params::use_case::AllUserLoginParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a JWT session token or error of type
    /// [crate::errors::use_case::AllUserLoginError] inside.
    /// 
    pub async fn login<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AllUserLoginParams<'a>,
        database_client: A
    ) -> Result<String, crate::errors::use_case::AllUserLoginError> {
        use crate::utils::{
            password_hash::verify_password,
            jwt_token::generate as generate_token
        };
        use crate::repositories::UserRepository;
        use crate::params::repository::UserRetrieveByLoginParams;
        use crate::errors::use_case::AllUserLoginError as Error;

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
    /// 1. params: [crate::params::use_case::AllUserRegisterParams] - params needed for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    ///
    /// ### Return type
    /// Returns result with either a fetched user or error of type
    /// [crate::errors::use_case::AllUserRegisterError] inside.
    ///
    pub async fn register<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::AllUserRegisterParams<'a>,
        database_client: A
    ) -> Result<(), crate::errors::use_case::AllUserRegisterError> {
        use crate::utils::password_hash::hash_password;
        use crate::repositories::UserRepository;
        use crate::params::repository::UserInsertParams;
        use crate::errors::use_case::AllUserRegisterError as Error;

        let mut database_client = database_client.acquire()
            .await
            .unwrap();
        let password_hash = hash_password(params.password).unwrap();
        
        UserRepository::insert(UserInsertParams { login: params.login, password_hash: &password_hash }, &mut *database_client)
            .await
            .map(|_| ())
            .map_err(|_| Error::AlreadyExists)
    }
}
