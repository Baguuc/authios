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
        use crate::params::repository::GroupPermissionDeleteParamsBuilder as ParamsBuilder;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        // authorize
        {
            use crate::params::use_case::UserAuthorizeParamsBuilder as ParamsBuilder;

            let params = ParamsBuilder::new()
                .set_token(params.token)
                .set_encryption_key(params.encryption_key)
                .set_permission_name(String::from("authios:all"))
                .build()
                .unwrap();
            
            match UsersUseCase::authorize(params, &mut *client).await {
                Ok(true) => (),
                _ => return Err(Error::Unauthorized)
            };
        }
        
        // check if the permission exist
        {
            use crate::params::repository::PermissionRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.permission_name.clone())
                .build()
                .unwrap();

            let _ = PermissionsRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::PermissionNotFound)?;
        }

        

        // check if the group exist
        let group = {
            use crate::params::repository::GroupRetrieveParamsBuilder as ParamsBuilder;
            
            let params = ParamsBuilder::new()
                .set_name(params.group_name.clone())
                .build()
                .unwrap();

            let group = GroupsRepository::retrieve(params, &mut *client)
                .await
                .map_err(|_| Error::GroupNotFound)?;

            group
        };
        
        
        
        // check if the permission is added
        if !group.permissions.contains(&params.permission_name) {
            return Err(Error::NotAddedYet);
        }

        

        let params = ParamsBuilder::new()
            .set_group_name(params.group_name)
            .set_permission_name(params.permission_name)
            .build()
            .unwrap();
        
        let _ = GroupPermissionsRepository::delete(params, &mut *client)
            .await;
        
        return Ok(());
    }
}
