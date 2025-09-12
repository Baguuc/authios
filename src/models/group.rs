/// # Group
///
/// Represents a single group.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug, sqlx::FromRow, Hash)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}
