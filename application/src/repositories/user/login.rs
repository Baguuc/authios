use crate::prelude::*;

impl crate::UserRepository {
    pub async fn login<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(login: &String, pwd: &String, encoding_key: String, client: C) -> Result<String> {
        let mut client = client
            .acquire()
            .await?;
        
        let user = Self::retrieve(login, &mut *client)
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
