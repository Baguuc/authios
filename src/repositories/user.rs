pub struct UserRepository;

impl UserRepository {
    /// ### Description
    /// Retrieve a single user from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserRetrieveParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the user is not found
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserRetrieveParams<'a>,
        database_client: A
    ) -> Option<crate::models::User> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("SELECT id, login, password_hash FROM users WHERE id = $1;")
            .bind(params.id)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Retrieve a single user from the database searching by login
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserRetrieveByLoginParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Options where None means the user is not found
    ///
    pub async fn retrieve_by_login<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserRetrieveByLoginParams<'a>,
        database_client: A
    ) -> Option<crate::models::User> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query_as("SELECT id, login, password_hash FROM users WHERE login = $1;")
            .bind(params.login)
            .fetch_one(&mut *database_client)
            .await
            .ok();

        result
    }   
    
    /// ### Description
    /// Insert a single user into the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserInsertParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Option where None means the user cannot be inserted because it violated the unique
    /// login constraint
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserInsertParams<'a>,
        database_client: A
    ) -> Result<crate::models::User, sqlx::Error> {
        let mut database_client = database_client.acquire()
            .await?;

        let _ = params.login;
        let result = sqlx::query_as("INSERT INTO users (login, password_hash) VALUES ($1, $2) RETURNING id, login, password_hash;")
            .bind(params.login)
            .bind(params.password_hash)
            .fetch_one(&mut *database_client)
            .await;

        result
    }
    
    /// ### Description
    /// Delete a user from the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserDeleteParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Option where None means the user is not found
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserDeleteParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query("DELETE FROM users WHERE id = $1;")
            .bind(params.id)
            .execute(&mut *database_client)
            .await
            .ok()?;

        if result.rows_affected() > 0 {
            Some(())
        } else {
            None
        }
    }
    
    /// ### Description
    /// Update a user in the database
    ///
    /// ### Arguments
    /// 1. params: [crate::params::repository::UserUpdateParams] - params for the
    ///    operation
    /// 2. database_client: [sqlx::Acquire] - the sqlx client connected to
    ///    postgres database
    /// 
    /// ### Return type
    /// Returns Option where None means the user is not found
    ///
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::UserUpdateParams<'a>,
        database_client: A
    ) -> Option<()> {
        let mut database_client = database_client.acquire()
            .await
            .ok()?;

        let result = sqlx::query("UPDATE users SET login = $1, password_hash = $2 WHERE id = $3;")
            .bind(params.login)
            .bind(params.password_hash)
            .bind(params.id)
            .execute(&mut *database_client)
            .await
            .ok()?;

        if result.rows_affected() > 0 {
            Some(())
        } else {
            None
        }
    }
}
