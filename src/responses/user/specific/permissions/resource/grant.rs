/// represents one of possible responses returned from granting user a resource permission (POST /users/{user_id}/permissions/resource)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserGrantResourcePermissionResponse {
    /// success
    Ok,
    /// user with specified id is not found
    UserNotFound,
    /// the permission to grant doesn't exist
    PermissionNotFound,
    /// the permission is already granted to the user
    AlreadyAdded,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
