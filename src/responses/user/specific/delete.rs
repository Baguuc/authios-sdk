/// represents one of possible responses returned from deleting a user as admin (DELETE /users/{})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserDeleteResponse {
    /// ok
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
