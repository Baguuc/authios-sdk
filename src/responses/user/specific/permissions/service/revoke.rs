/// represents one of possible responses returned from revoking a service permission from a user (DELETE /users/{user_id}/permissions/service/{service_id})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserRevokeServicePermissionResponse {
    /// success
    Ok,
    /// user with specified id is not found
    UserNotFound,
    /// the permission to revoke doesn't exist
    PermissionNotFound,
    /// the permission is not yet granted to the user
    NotAddedYet,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
