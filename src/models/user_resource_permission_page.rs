/// Represents a page of user resource permission list with total page count and page number.
///
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserResourcePermissionPage { 
    pub page_number: u32,
    /// when None it means that the page is too big
    pub permissions: Option<Vec<crate::models::UserResourcePermission>>
}
