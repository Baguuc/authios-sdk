/// represents one of possible responses returned from revoking a resource permissions from a user (DELETE /users/{user_id}/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserRevokeResourcePermissionResponse {
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
