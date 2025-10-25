/// represents one of possible responses returned from validating root user password (GET /users/root/password)
///
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum RootUserCheckPasswordResponse {
    // success, bool indicating if user provided correct password returned 
    Ok {
        authorized: bool
    },
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
