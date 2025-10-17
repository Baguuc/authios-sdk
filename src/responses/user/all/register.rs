/// represents one of possible responses returned from register route (POST /users)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum AllUserRegisterResponse {
    /// success
    Ok,
    /// user with specified login already exists
    AlreadyExists,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
