/// # hash_password
///
/// Hash a password with Argon2 algorithm (just a shorthand).
///
/// ### Arguments:
/// password: [std::string::String]
///
/// Returns eighter the password hash as a [std::string::String] or the error that occured
/// while hashing.
///
pub fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    use argon2::{
        Argon2,
        PasswordHasher,
        password_hash::{
            SaltString,
            rand_core::OsRng
        }
    };
    
    let pwd = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(pwd, &salt)?
        .to_string();

    return Ok(password_hash);
}

/// # verify_password
///
/// Check if a password matches a password hash
///
/// ### Arguments:
/// + password: &[std::string::String]
/// + password_hash: &[std::string::String]
///
/// Returns a boolean that indicates if the hash matches the password.
///
pub fn verify_password(password: &String, password_hash: &String) -> bool {
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
