/// represents one of possible responses returned from deleting a resource permission (DELETE
/// /permissions/resource)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ResourcePermissionDeleteResponse {
    /// ok
    Ok,
    /// permission matching provided criteria already exists
    AlreadyExists,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
