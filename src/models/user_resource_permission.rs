/// Represents a single resource permission binded to a user.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserResourcePermission {
    pub service_id: String,
    pub resource_type: String,
    pub resource_id: i32,
    pub permissions: Vec<String>
}
