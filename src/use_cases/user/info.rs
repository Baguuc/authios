use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::info
    ///
    /// retrieve a user from JWT token, checking for possible errors
    ///
    /// Errors:
    /// + when the provided token is invalid;
    /// + when a user login specified in the token not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::UserInfoParams,
        client: A
    ) -> Result<crate::models::User, crate::errors::UserInfoError> {
        use crate::repositories::UsersRepository; 
        use crate::errors::UserInfoError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;

        let data = UsersRepository::retrieve(&claims.sub, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        return Ok(data);
    }
}
