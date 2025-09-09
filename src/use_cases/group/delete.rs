use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
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
        params: crate::params::use_case::GroupDeleteParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupDeleteError> {
        use crate::repositories::GroupsRepository;
        use crate::errors::use_case::GroupDeleteError as Error;
        use crate::params::repository::GroupDeleteParamsBuilder as ParamsBuilder;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let params = ParamsBuilder::new()
            .set_name(params.name)
            .build()
            .unwrap();

        // this won't error so we can skip this result
        let result = GroupsRepository::delete(params, &mut *client)
            .await
            .unwrap();

        if result.rows_affected() == 0 {
            return Err(Error::NotExist);
        }
        
        return Ok(());
    }
}
