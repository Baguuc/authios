impl crate::GroupsUseCase {
    /// # GroupsUseCase::retrieve
    ///
    /// retrieve a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<authios_domain::Group, GroupRetrieveError> {
        type Error = GroupRetrieveError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let data = crate::GroupsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(data);
    }
}

pub enum GroupRetrieveError {
    Generic
}

impl ToString for GroupRetrieveError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
