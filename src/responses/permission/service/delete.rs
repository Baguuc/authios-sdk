/// represents one of possible responses returned from deleting a service permission (DELETE
/// /permissions/service)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ServicePermissionDeleteResponse {
    /// ok
    Ok,
    /// permission matching provided criteria is not found
    PermissionNotFound,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
