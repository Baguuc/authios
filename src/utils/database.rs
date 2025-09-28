/// # create_pool
///
/// creates a database pool from database credentials wrapped in [crate::config::DatabaseConfig]
/// struct.
pub async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
