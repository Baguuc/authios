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
    pub async fn register<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, pwd: &String, client: C) -> Result<(), UserRegisterError> {
        use crate::UsersRepository; 
        
        type Error = UserRegisterError; 
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let password_hash = match crate::utils::hash_password(pwd.clone()) {
            Ok(h) => h,
            Err(_) => return Err(Error::CannotHashPassword)
        };

        let _ = UsersRepository::insert(login, &password_hash, &mut *client).await
            .map_err(|_| Error::AlreadyExist)?;
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserRegisterError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("CANNOT_HASH_PASSWORD")]
    CannotHashPassword,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
