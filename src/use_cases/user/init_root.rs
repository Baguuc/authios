use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::init_root
    ///
    /// init the root account and all of related data checking for errors
    ///
    /// Errors:
    /// + when a permission named "authios:all" already exists;
    /// + when a group named "root" already exists;
    /// + when a user named "root" already exists;
    /// + when provided password cannot be hashed;
    /// + when database connection cannot be acquired;
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
                .await
                .map_err(|_| Error::PermissionExists)?;
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
                .await
                .map_err(|_| Error::GroupExists)?;
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
                .await
                .map_err(|_| Error::GroupExists)?;
        }

        let _ = tx.commit().await;

        return Ok(());
    }
}
