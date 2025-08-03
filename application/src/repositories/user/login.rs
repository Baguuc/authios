use crate::prelude::*;

impl crate::UserRepository {
    pub async fn login<'c, C: sqlx::postgres::PgExecutor<'c>>(login: &String, pwd: &String, encoding_key: String, client: C) -> Result<String> {
        let user = Self::retrieve(login, client)
            .await?;

        if !Self::verify_password(pwd, &user.pwd) {
            return Err(Error::Generic(String::from("Wrong password")));
        };
        
        let token = Self::generate_jwt(
            user.login,
            (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
            encoding_key
        )?;
        
        return Ok(token);
    }
}
