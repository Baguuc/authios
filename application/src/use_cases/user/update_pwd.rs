impl crate::UsersUseCase {
    /// # UsersUseCase::update_pwd
    ///
    /// update user's password, hashing password and checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login already exist;
    /// + when the provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(user_login: &String, pwd: &String, client: A) -> Result<(), UserUpdatePwdError> {
        type Error = UserUpdatePwdError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let pwd = crate::utils::password_hash::hash_password(pwd.clone())
            .map_err(|_| Error::Generic)?;
        
        crate::UsersRepository::update(user_login, &pwd, &mut *client)
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
