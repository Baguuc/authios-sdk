/// represents one of possible responses returned from checking a user's service permission (GET /users/{user_id}/permissions/service/{service_id})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserCheckServicePermissionResponse {
    /// ok, bool indicating if user has permission returned
    Ok {
        has_permission: bool
    },
    /// user with specified id is not found
    UserNotFound,
    /// the permission to check for doesn't exist
    PermissionNotFound,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
