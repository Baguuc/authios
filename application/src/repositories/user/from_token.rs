use crate::prelude::*;

impl crate::UserRepository {
    pub async fn from_token<'c, C: sqlx::postgres::PgExecutor<'c>>(token: &String, encoding_key: &String, client: C) -> Result<authios_domain::User> {
        let claims = Self::get_claims(token, encoding_key)?;
        let user = Self::retrieve(&claims.sub, client)
            .await?;
        
        return Ok(user);
    }
}
