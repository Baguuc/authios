use crate::prelude::*;

impl crate::UserRepository {
    pub fn hash_password(password: String) -> Result<String> {
        use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
        
        let pwd = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = match Argon2::default().hash_password(pwd, &salt) {
            Ok(hash) => hash,
            Err(err) => return Err(Error::Generic(err.to_string())),
        }
        .to_string();

        return Ok(password_hash);
    }
}
