/// Represents a page of user ids after fetching by resource permission
///
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UsersPage { 
    pub page_number: u32,
    /// when None it means that the page is too big
    pub users: Option<Vec<crate::models::User>>
}
