/// represents one of possible responses returned from checking logged user's resource permissions (GET /users/me/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum LoggedUserCheckResourcePermissionResponse {
    /// ok, bool indicating if user has permission returned
    Ok {
        has_permission: bool
    },
    /// token sent is invalid - in wrong format or pointing to a user that doesn't exist
    InvalidToken,
    /// the permission to check for doesn't exist
    PermissionNotFound,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
