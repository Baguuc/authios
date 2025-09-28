/// Represents a single resource permission.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct ResourcePermission {
    pub id: i32,
    pub service_id: String,
    pub resource_type: String,
    pub permission_name: String
}
