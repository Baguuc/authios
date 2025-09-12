/// # User
///
/// Represents a single user.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub login: String,
    pub pwd: String,
    pub groups: Vec<String>
}

/// # Claims
///
/// Represents claims of the JWT tokens. Used for encryption/decryption of the tokens.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}
