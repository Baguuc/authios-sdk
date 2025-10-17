/// represents one of possible responses returned from creating a service permission (POST
/// /permissions/service)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ServicePermissionCreateResponse {
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
