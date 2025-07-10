#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct Permission {
    pub name: String,
}
