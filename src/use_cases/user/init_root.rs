use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::init_root
    ///
    /// Init the root account and all of related data checking for errors
    ///
    /// ### Arguments:
    /// + params: [crate::params::use_case::user::init_root::UserInitRootParams] - the parameters of the query
    /// + client: [sqlx::Acquire] - sqlx postgres client
    /// 
    /// ### Errors:
    /// Errors described in [crate::errors::use_case::user::init_root::UserInitRootError]
    ///
    pub async fn init_root<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserInitRootParams,
        client: A
    ) -> Result<(), crate::errors::use_case::UserInitRootError> { 
        use crate::errors::use_case::UserInitRootError as Error;

        let mut tx = client
            .begin()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        // create the root permission 
        {
            use crate::params::repository::PermissionInsertParamsBuilder as ParamsBuilder;
            use crate::repositories::PermissionsRepository;

            let params = ParamsBuilder::new()
                .set_name(String::from("authios:all"))
                .build()
                .unwrap();

            let _ = PermissionsRepository::insert(params, &mut *tx)
                .await;
        }
        
        // create the root group 
        {
            use crate::params::repository::GroupInsertParamsBuilder as ParamsBuilder;
            use crate::repositories::GroupsRepository;

            let params = ParamsBuilder::new()
                .set_name(String::from("root"))
                .build()
                .unwrap();

            let _ = GroupsRepository::insert(params, &mut *tx)
                .await;
        }
        
        // create the root user 
        {
            use crate::params::repository::UserInsertParamsBuilder as ParamsBuilder;
            use crate::repositories::UsersRepository;
            use crate::utils::password_hash::hash_password;

            let pwd = hash_password(params.pwd)
                .map_err(|_| Error::CannotHashPassword)?;

            let params = ParamsBuilder::new()
                .set_login(String::from("root"))
                .set_pwd(pwd)
                .build()
                .unwrap();

            let _ = UsersRepository::insert(params, &mut *tx)
                .await;
        }

        let _ = tx.commit().await;

        return Ok(());
    }
}
