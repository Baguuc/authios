#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub login: String,
    pub pwd: String,
    pub groups: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}
