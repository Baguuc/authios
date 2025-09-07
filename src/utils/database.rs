const MIGRATIONS: [&str; 10] = [
"CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  login VARCHAR(32) NOT NULL UNIQUE,
  pwd VARCHAR(97) NOT NULL
);",

"CREATE INDEX IF NOT EXISTS users_login_index ON users USING HASH (login);",

"CREATE TABLE IF NOT EXISTS groups (name TEXT PRIMARY KEY);",

"CREATE TABLE IF NOT EXISTS user_groups (
  user_login TEXT NOT NULL,
  group_name TEXT NOT NULL,

  FOREIGN KEY (user_login) REFERENCES users(login) ON DELETE CASCADE,
  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE
);",

"CREATE TABLE IF NOT EXISTS permissions (name TEXT PRIMARY KEY);",

"CREATE TABLE IF NOT EXISTS group_permissions (
  group_name TEXT NOT NULL,
  permission_name TEXT NOT NULL,

  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE,
  FOREIGN KEY (permission_name) REFERENCES permissions(name) ON DELETE CASCADE
);",

"ALTER TABLE group_permissions DROP CONSTRAINT IF EXISTS group_permissions_pair_unique;",

"ALTER TABLE group_permissions ADD CONSTRAINT group_permissions_pair_unique UNIQUE (group_name, permission_name);",

"ALTER TABLE user_groups DROP CONSTRAINT IF EXISTS user_groups_pair_unique;",

"ALTER TABLE user_groups ADD CONSTRAINT user_groups_pair_unique UNIQUE (user_login, group_name);"
];

pub async fn migrate(client: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    use sqlx::query;
    use clin::components::{progress_bar, error};

    let mut tx = client.begin().await?;

    let count = MIGRATIONS.len();
    let mut curr_count = 0;

    progress_bar(count, curr_count);
    
    for sql in MIGRATIONS {
        curr_count += 1;

        match query(sql).execute(&mut *tx).await {
            Ok(_) => {},
            Err(_) => {
                error("Migration command failed.", format!("SQL:\n{}", sql));
                
                std::process::exit(1);
            }
        }; 

        progress_bar(count, curr_count);
    }

    let _ = tx.commit().await?;
    
    return Ok(());
}

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
