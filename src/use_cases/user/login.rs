use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::login
    ///
    /// Log user in and return the session token, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::login::UserLoginParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::login::UserLoginError]
    ///
    pub async fn login<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserLoginParams,
        client: C
    ) -> Result<String, crate::errors::use_case::UserLoginError> {
        use crate::repositories::UsersRepository; 
        use crate::utils::password_hash::verify_password;
        use crate::utils::jwt_token::generate;
        use crate::errors::use_case::UserLoginError as Error; 
        use crate::params::repository::UserRetrieveParams as Params;
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // retrieve the user
        let user = {
            UsersRepository::retrieve(Params { login: params.login }, &mut *client)
                .await
                .map_err(|_| Error::UserNotFound)?
        };

        if !verify_password(&params.pwd, &user.pwd) {
            return Err(Error::WrongPassword);
        };
        
        let token = generate(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            params.encryption_key
        ).map_err(|_| Error::CannotGenerateToken)?;
        
        return Ok(token);
    }
}
