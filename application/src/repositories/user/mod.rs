use crate::prelude::*;

pub struct UserRepository;

impl UserRepository {    
    fn verify_password(password: &String, password_hash: &String) -> bool {
        use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::Encoding};

        let password_hash = match PasswordHash::parse(password_hash.as_str(), Encoding::B64) {
            Ok(hash) => hash,
            Err(_) => return false
        };

        let _ = match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
            Ok(_) => return true,
            Err(_) => return false
        };
    }
    
    fn generate_jwt(login: String, expires: usize, key: String) -> Result<String> {
        use jsonwebtoken::{encode, Header, EncodingKey};

        let claims = authios_domain::Claims {
            sub: login,
            exp: expires,
        };

        let encoded = encode(
            &Header::default(),
            &claims, 
            &EncodingKey::from_secret(key.as_ref())
        )?;

        return Ok(encoded);
    }
    
    fn get_claims(token: &String, encoding_key: &String) -> Result<authios_domain::Claims> {
        use jsonwebtoken::{decode, DecodingKey, Validation};
        
        let decoded = decode::<authios_domain::Claims>(
            token,
            &DecodingKey::from_secret(encoding_key.as_ref()),
            &Validation::default()
        )?;

        return Ok(decoded.claims);
    }
}
