impl crate::UsersUseCase {
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
    pub async fn login<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, pwd: &String, encoding_key: String, client: C) -> Result<String, UserLoginError> {
        use crate::UsersRepository; 
        use crate::utils::password_hash::verify_password;
        use crate::utils::jwt_token::generate;
        
        type Error = UserLoginError; 
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let user = UsersRepository::retrieve(login, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;

        if !verify_password(pwd, &user.pwd) {
            return Err(Error::InvalidCredentials);
        };
        
        let token = generate(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            encoding_key
        ).map_err(|_| Error::CannotGenerateToken)?;
        
        return Ok(token);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserLoginError {
    #[error("NOT_EXIST")]
    NotExist,
    #[error("INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[error("CANNOT_GENERATE_TOKEN")]
    CannotGenerateToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
