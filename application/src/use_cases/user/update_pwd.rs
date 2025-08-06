impl crate::UsersUseCase {
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
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(token: &String, encoding_key: &String, pwd: &String, client: A) -> Result<(), UserUpdatePwdError> {
        use crate::use_cases::user::retrieve_from_token::UserRetrieveFromTokenError;
        
        type Error = UserUpdatePwdError;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let user = crate::UsersUseCase::retrieve_from_token(token, encoding_key, &mut *client)
            .await
            .map_err(|error| match error {
                 UserRetrieveFromTokenError::InvalidToken => Error::InvalidToken,
                 UserRetrieveFromTokenError::NotExist => Error::NotExist,
                 UserRetrieveFromTokenError::DatabaseConnection => Error::DatabaseConnection,
            })?;
        
        let pwd = crate::utils::password_hash::hash_password(pwd.clone())
            .map_err(|_| Error::CannotHash)?;
        
        let _ = crate::UsersRepository::update(&user.login, &pwd, &mut *client)
            .await;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserUpdatePwdError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("CANNOT_HASH")]
    CannotHash,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
