/// Represents a page of user ids after fetching by resource permission
///
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsersPage { 
    pub total_page_count: u32,
    pub page_number: u32,
    pub users: Vec<crate::models::User>
}
