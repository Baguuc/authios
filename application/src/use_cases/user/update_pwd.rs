impl crate::UsersUseCase {
    /// # UsersUseCase::update_pwd
    ///
    /// update user's password, hashing password and checking for possible errors
    ///
    /// Errors:
    /// + when provided token is invalid;
    /// + when a user with provided token already exist;
    /// + when the provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(token: &String, encoding_key: &String, pwd: &String, client: A) -> Result<(), UserUpdatePwdError> {
        type Error = UserUpdatePwdError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let user = crate::UsersUseCase::retrieve_from_token(token, encoding_key, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        let pwd = crate::utils::password_hash::hash_password(pwd.clone())
            .map_err(|_| Error::Generic)?;
        
        crate::UsersRepository::update(&user.login, &pwd, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(());
    }
}

pub enum UserUpdatePwdError {
    Generic
}

impl ToString for UserUpdatePwdError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
