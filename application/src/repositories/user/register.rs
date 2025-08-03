use crate::prelude::*;

impl crate::UserRepository {
    pub async fn register<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, client: C) -> Result<()> { 
        let pwd = Self::hash_password(pwd.clone())?;

        let _ = Self::insert(login, &pwd, client)
            .await?;
        
        return Ok(());
    }
}
