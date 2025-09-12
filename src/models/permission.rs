/// # Permission
///
/// Represents a single permission.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct Permission {
    pub name: String,
}
