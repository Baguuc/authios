/// params needed to retrieve a user from the database
///
pub struct UserRetrieveParams<'p> {
    /// id of the user to retrieve
    ///
    pub id: &'p i32
}

/// params needed to retrieve a user from the database
///
pub struct UserRetrieveByLoginParams<'p> {
    /// id of the user to retrieve
    ///
    pub login: &'p String
}

/// params needed to insert user to the database
///
pub struct UserInsertParams<'p> {
    /// login of the user to insert
    ///
    pub login: &'p String,
    /// password hash of the user to insert
    ///
    pub password_hash: &'p String
}

/// params needed to delete a user from the database
///
pub struct UserDeleteParams<'p> {
    /// id of the user to delete
    ///
    pub id: &'p i32
}

/// params needed to update a user in the database
///
pub struct UserUpdateParams<'p> {
    /// id of the user to update
    ///
    pub id: &'p i32,
    /// new login of the user
    ///
    pub login: &'p String,
    /// new password hash of the user
    ///
    pub password_hash: &'p String
}
