#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug, sqlx::FromRow, Hash)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}
