use crate::prelude::*;

impl crate::UserRepository {
    pub async fn register<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, pwd: &String, client: C) -> Result<()> { 
        let mut client = client
            .acquire()
            .await?;
        
        let pwd = Self::hash_password(pwd.clone())?;

        let _ = Self::insert(login, &pwd, &mut *client)
            .await?;
        
        return Ok(());
    }
}
