impl crate::GroupsUseCase {
    /// # GroupsUseCase::delete
    ///
    /// delete a group, checking for possible errors
    ///
    /// Errors:
    /// + when a group with provided name do not exist;
    /// + when database connection cannot be acquired;
    /// + when the user is not authorized for this operation;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: authios_domain::GroupDeleteParams,
        client: A
    ) -> Result<(), authios_domain::GroupDeleteError> {
        type Error = authios_domain::GroupDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let _ = crate::GroupsRepository::retrieve(&params.name, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        // this won't error so we can skip this result
        let _ = crate::GroupsRepository::delete(&params.name, &mut *client)
            .await;
        
        return Ok(());
    }
}
