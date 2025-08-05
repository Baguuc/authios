impl crate::GroupsUseCase {
    /// # GroupsUseCase::delete
    ///
    /// delete a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name do not exist;
    /// + when database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<(), GroupDeleteError> {
        type Error = GroupDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::Generic)?;
        
        let _ = crate::GroupsRepository::retrieve(name, &mut *client)
            .await
            .map_err(|_| Error::Generic)?;
        
        // this won't error so we can skip this result
        let _ = crate::GroupsRepository::delete(name, &mut *client)
            .await;
        
        return Ok(());
    }
}

pub enum GroupDeleteError {
    Generic
}

impl ToString for GroupDeleteError {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Generic => String::from("GENERIC")
        };
    }
}
