/// Represents a page of user resource permission list with total page count and page number.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserResourcePermissionPage { 
    pub total_page_count: u32,
    pub page_number: u32,
    pub permissions: Vec<crate::models::UserResourcePermission>
}
