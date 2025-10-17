/// represents one of possible responses returned from updating user's data as admin (PATCH /users/{user_id})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserUpdateResponse {
    /// success
    Ok,
    /// user with specified id is not found
    UserNotFound,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
