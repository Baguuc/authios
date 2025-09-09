use crate::use_cases::UsersUseCase;

impl UsersUseCase {
    /// # UsersUseCase::register
    ///
    /// log user in and return the session token, checking for possible errors
    ///
    /// Errors:
    /// + when a user with provided login already exist;
    /// + when provided password cannot be hashed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn register<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(
        params: crate::params::use_case::UserRegisterParams,
        client: C
    ) -> Result<(), crate::errors::use_case::UserRegisterError> {
        use crate::utils::password_hash::hash_password;
        use crate::repositories::UsersRepository; 
        use crate::errors::use_case::UserRegisterError as Error; 
        use crate::params::repository::UserInsertParamsBuilder as ParamsBuilder;
        
        let mut client = client
            .acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let password_hash = match hash_password(params.pwd.clone()) {
            Ok(h) => h,
            Err(_) => return Err(Error::CannotHashPassword)
        };

        // insert the user
        {
            let params = ParamsBuilder::new()
                .set_login(params.login)
                .set_pwd(password_hash)
                .build()
                .unwrap();

            let _ = UsersRepository::insert(params, &mut *client).await
                .map_err(|_| Error::AlreadyExist)?;
        }

        return Ok(());
    }
}
