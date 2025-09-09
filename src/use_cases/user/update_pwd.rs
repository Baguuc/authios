use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::update_pwd
    ///
    /// update user's password, hashing password and checking for possible errors
    ///
    /// Errors:
    /// + when provided token is invalid;
    /// + when a user with provided token do not exist;
    /// + when the provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserUpdatePwdParams,
        client: A
    ) -> Result<(), crate::errors::use_case::UserUpdatePwdError> {
        use crate::repositories::UsersRepository;
        use crate::errors::use_case::UserUpdatePwdError as Error;
        use crate::params::repository::UserUpdateParamsBuilder as ParamsBuilder;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // retrieve user login
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_login = claims.sub;
        
        // hash the password
        let pwd = crate::utils::password_hash::hash_password(params.new_pwd.clone())
            .map_err(|_| Error::CannotHash)?;
        
        let params = ParamsBuilder::new()
            .set_user_login(user_login)
            .set_new_pwd(pwd)
            .build()
            .unwrap();

        let result = UsersRepository::update(params, &mut *client)
            .await
            .unwrap();

        if result.rows_affected() == 0 {
            // no effect so user not found
            return Err(Error::NotExist);
        }
        
        return Ok(());
    }
}
