impl crate::UsersUseCase {
    /// # UsersUseCase::info
    ///
    /// retrieve a user from JWT token, checking for possible errors
    ///
    /// Errors:
    /// + when the provided token is invalid;
    /// + when a user login specified in the token not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(token: &String, encoding_key: &String, client: A) -> Result<authios_domain::User, UserInfoError> {
        use crate::UsersRepository; 

        type Error = UserInfoError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let claims = crate::utils::jwt_token::get_claims(token, encoding_key)
            .map_err(|_| Error::InvalidToken)?;

        let data = UsersRepository::retrieve(&claims.sub, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserInfoError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("NotExist")]
    NotExist,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
