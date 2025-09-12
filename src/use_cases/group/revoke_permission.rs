use crate::use_cases::GroupsUseCase;

impl GroupsUseCase {
    /// # GroupsUseCase::revoke
    ///
    /// Revoke a permission from a group, checking for possible errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::group::revoke_permission::GroupRevokePermissionParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::group::GroupRevokePermissionError]
    ///
    pub async fn revoke_permission<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::GroupRevokePermissionParams,
        client: A
    ) -> Result<(), crate::errors::use_case::GroupRevokePermissionError> {
        use crate::repositories::{
            PermissionsRepository,
            GroupsRepository,
            GroupPermissionsRepository
        };
        use crate::use_cases::UsersUseCase;
        use crate::errors::use_case::GroupRevokePermissionError as Error;
        use crate::params::repository::GroupPermissionDeleteParams as Params;

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
        
        // check if the permission exist
        {
            use crate::params::repository::PermissionRetrieveParams as Params;

            let _ = PermissionsRepository::retrieve(Params { name: params.permission_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::PermissionNotFound)?;
        }

        

        // check if the group exist
        let group = {
            use crate::params::repository::GroupRetrieveParams as Params;

            let group = GroupsRepository::retrieve(Params { name: params.group_name.clone() }, &mut *client)
                .await
                .map_err(|_| Error::GroupNotFound)?;

            group
        };
        
        
        
        // check if the permission is added
        if !group.permissions.contains(&params.permission_name) {
            return Err(Error::NotAddedYet);
        }        
        
        let _ = GroupPermissionsRepository::delete(Params { group_name: params.group_name, permission_name: params.permission_name }, &mut *client)
            .await;
        
        return Ok(());
    }
}
