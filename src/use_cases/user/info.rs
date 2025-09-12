use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::info
    ///
    /// Retrieve a user from JWT token, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::info::UserInfoParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::info::UserInfoError]
    ///
    pub async fn info<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserInfoParams,
        client: A
    ) -> Result<crate::models::User, crate::errors::use_case::UserInfoError> {
        use crate::repositories::UsersRepository; 
        use crate::errors::use_case::UserInfoError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // get the user login from the token
        let claims = crate::utils::jwt_token::get_claims(&params.token, &params.encryption_key)
            .map_err(|_| Error::InvalidToken)?;
        let user_login = claims.sub;

        // get the data
        let data = {
            use crate::params::repository::UserRetrieveParams as Params;

            // invalid token points to non-existent user
            UsersRepository::retrieve(Params { login: user_login }, &mut *client)
                .await
                .map_err(|_| Error::InvalidToken)?
        };
        
        return Ok(data);
    }
}
