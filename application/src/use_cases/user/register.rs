impl crate::UsersUseCase {
    /// # UsersUseCase::register
    ///
    /// log user in and return the session token, checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login already exist;
    /// + when provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn register<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: authios_domain::UserRegisterParams,
        client: C
    ) -> Result<(), authios_domain::UserRegisterError> {
        use crate::UsersRepository; 
        
        type Error = authios_domain::UserRegisterError; 
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let password_hash = match crate::utils::hash_password(params.pwd.clone()) {
            Ok(h) => h,
            Err(_) => return Err(Error::CannotHashPassword)
        };

        let _ = UsersRepository::insert(&params.login, &password_hash, &mut *client).await
            .map_err(|_| Error::AlreadyExist)?;
        
        return Ok(());
    }
}
