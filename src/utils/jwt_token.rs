pub struct JWTToken;

pub fn generate(login: String, expires: usize, key: String) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{encode, Header, EncodingKey};

    let claims = crate::models::Claims {
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

pub fn get_claims(token: &String, encoding_key: &String) -> Result<crate::models::Claims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    
    let decoded = decode::<crate::models::Claims>(
        token,
        &DecodingKey::from_secret(encoding_key.as_ref()),
        &Validation::default()
    )?;

    return Ok(decoded.claims);
}
