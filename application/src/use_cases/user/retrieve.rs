impl crate::UsersUseCase {
    /// # UsersUseCase::retrieve
    ///
    /// retrieve a user, checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(login: &String, client: A) -> Result<authios_domain::User, UserRetrieveError> {
        type Error = UserRetrieveError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let data = crate::UsersRepository::retrieve(login, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(data);
    }
}

pub enum UserRetrieveError {
    Generic
}

impl ToString for UserRetrieveError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
