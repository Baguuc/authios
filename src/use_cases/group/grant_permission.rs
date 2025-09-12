use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
    /// # GroupsUseCase::grant_permission
    ///
    /// Grant a permission to a group, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::group::grant_permission::GroupGrantPermissionParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::group::GroupGrantPermissionError]
    ///
    pub async fn grant_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::GroupGrantPermissionParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupGrantPermissionError> {
        use crate::repositories::{
            PermissionsRepository,
            GroupsRepository,
            GroupPermissionsRepository
        };
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::GroupGrantPermissionError as Error;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParams as Params;

            let params = Params {
                encryption_key: params.encryption_key,
                permission_name: String::from("authios:all"),
                token: params.token
            };
            
            match UsersUseCase::authorize(params, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }

        // check if the permission exist
        {
            use crate::params::repository::PermissionRetrieveParams as Params;

            let _ = PermissionsRepository::retrieve(Params { name: params.permission_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::PermissionNotFound)?;
        }

        

        // check if the group exist
        {
            use crate::params::repository::GroupRetrieveParams as Params;

            let _ = GroupsRepository::retrieve(Params { name: params.group_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::GroupNotFound)?;
        }


        // add the permission to the group
        {
            use crate::params::repository::GroupPermissionInsertParams as Params;
            
            GroupPermissionsRepository::insert(Params { group_name: params.group_name, permission_name: params.permission_name }, &mut *client)
                .await
                // already added
                .map_err(|_| Error::AlreadyAdded)?;
        }

        return Ok(());
    }
}
