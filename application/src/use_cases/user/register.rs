impl crate::UsersUseCase {
    /// # UsersUseCase::register
    ///
    /// register a user, hashing password and checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login already exist;
    /// + when the provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn register<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(data: &authios_domain::User, client: A) -> Result<(), UserRegisterError> {
        type Error = UserRegisterError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let pwd = crate::utils::password_hash::hash_password(data.pwd.clone())
            .map_err(|_| Error::CannotHash)?;
        
        let data = authios_domain::User {
            login: data.login.clone(),
            pwd,
            // it will be skipped
            groups: vec![]
        };
        
        crate::UsersRepository::insert(&data, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?; 
        
        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserRegisterError {
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("CANNOT_HASH")]
    CannotHash,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
