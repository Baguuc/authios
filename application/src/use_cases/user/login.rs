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
        type Error = UserLoginError; 
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let user = Self::retrieve(login, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;

        if !crate::utils::password_hash::verify_password(pwd, &user.pwd) {
            return Err(Error::Generic);
        };
        
        let token = crate::utils::jwt_token::generate(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            encoding_key
        ).map_err(|_| Error::Generic)?;
        
        return Ok(token);
    }
}

pub enum UserLoginError {
    Generic
}

impl ToString for UserLoginError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
