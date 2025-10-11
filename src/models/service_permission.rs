#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct ServicePermission {
    pub id: i32,
    pub service_id: String
}
