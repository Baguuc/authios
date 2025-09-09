use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::login
    ///
    /// log user in and return the session token, checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login do not exist;
    /// + when provided password is invalid;
    /// + when the token cannot be generated;
    /// + when database connection cannot be acquired;
    ///
    pub async fn login<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserLoginParams,
        client: C
    ) -> Result<String, crate::errors::use_case::UserLoginError> {
        use crate::repositories::UsersRepository; 
        use crate::utils::password_hash::verify_password;
        use crate::utils::jwt_token::generate;
        use crate::errors::use_case::UserLoginError as Error; 
        use crate::params::repository::UserRetrieveParamsBuilder as ParamsBuilder;
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // retrieve the user
        let user = {
            let params = ParamsBuilder::new()
                .set_login(params.login.clone())
                .build()
                .unwrap();

            UsersRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::NotExist)?
        };

        if !verify_password(&params.pwd, &user.pwd) {
            return Err(Error::InvalidCredentials);
        };
        
        let token = generate(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            params.encryption_key
        ).map_err(|_| Error::CannotGenerateToken)?;
        
        return Ok(token);
    }
}
