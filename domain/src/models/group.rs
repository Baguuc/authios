#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug, sqlx::FromRow)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}
