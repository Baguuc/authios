/// # generate
///
/// Generate a JWT token based on user's login, and expiration date.
/// 
/// ### Arguments:
/// + login: [std::string::String]
/// + expires: [std::usize] (date when expires),
/// + key: [std::string::String] (system's JWT encryption key)
///
/// Returns eighter the generated token as a [std::string::String] or error.
///
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

/// # get_claims
///
/// ### Arguments:
/// + token: String,
/// + key: String (system's JWT encryption key)
///
// Returns eighter the generated token as a String or error.
//
pub fn get_claims(token: &String, key: &String) -> Result<crate::models::Claims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    
    let decoded = decode::<crate::models::Claims>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default()
    )?;

    return Ok(decoded.claims);
}
