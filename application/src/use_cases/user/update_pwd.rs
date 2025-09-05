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
    pub async fn update_pwd<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::UserUpdatePwdParams,
        client: A
    ) -> Result<(), UserUpdatePwdError> {
        type Error = UserUpdatePwdError;
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;

        let user = crate::UsersRepository::retrieve(&claims.sub, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        
        let pwd = crate::utils::password_hash::hash_password(params.new_pwd.clone())
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
