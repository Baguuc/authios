impl crate::GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// create a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name already exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(data: &authios_domain::Group, client: A) -> Result<(), GroupCreateError> {
        type Error = GroupCreateError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        crate::GroupsRepository::insert(data, &mut *client)
            .await
            .map_err(|_| Error::Generic)?; 
        
        return Ok(());
    }
}

pub enum GroupCreateError {
    Generic
}

impl ToString for GroupCreateError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
