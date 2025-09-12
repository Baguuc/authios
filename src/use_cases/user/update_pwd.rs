use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::update_pwd
    ///
    /// Update user's password, hashing password and checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::update_pwd::UserUpdatePwdParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::update_pwd::UserUpdatePwdError]
    ///
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserUpdatePwdParams,
        client: A
    ) -> Result<(), crate::errors::use_case::UserUpdatePwdError> {
        use crate::repositories::UsersRepository;
        use crate::errors::use_case::UserUpdatePwdError as Error;
        use crate::params::repository::UserUpdateParams as Params;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // retrieve user login
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_login = claims.sub;
        
        // hash the password
        let password_hash = crate::utils::password_hash::hash_password(params.new_pwd.clone())
            .map_err(|_| Error::CannotHash)?;

        let result = UsersRepository::update(Params { user_login, new_pwd: password_hash }, &mut *client)
            .await
            .unwrap();

        if result.rows_affected() == 0 {
            // no effect so user not found so the token is invalid as it points to non-existent
            // user
            return Err(Error::InvalidToken);
        }
        
        return Ok(());
    }
}
