use crate::prelude::*;

impl crate::UserRepository {
    pub async fn from_token<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(token: &String, encoding_key: &String, client: C) -> Result<authios_domain::User> {
        let mut client = client
            .acquire()
            .await?;
        
        let claims = Self::get_claims(token, encoding_key)?;
        let user = Self::retrieve(&claims.sub, &mut *client)
            .await?;
        
        return Ok(user);
    }
}
