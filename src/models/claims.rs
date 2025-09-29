/// # Claims
///
/// Represents claims of the JWT tokens. Used for encryption/decryption of the tokens.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize
}
