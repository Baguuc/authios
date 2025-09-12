use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
    /// # GroupsUseCase::create
    ///
    /// Create a group, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::group::create::GroupCreateParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::group::GroupCreateError]
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::GroupCreateParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupCreateError> {
        use crate::repositories::GroupsRepository;
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::GroupCreateError as Error; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParams as Params;
            
            match UsersUseCase::authorize(Params { token: params.token, encryption_key: params.encryption_key, permission_name: String::from("authios:all") }, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }

        // insert group
        {
            use crate::params::repository::GroupInsertParams as Params; 

            GroupsRepository::insert(Params { name: params.name }, &mut *client)
                .await
                .map_err(|_| Error::AlreadyExist)?;
        }
        
        return Ok(());
    }
}
