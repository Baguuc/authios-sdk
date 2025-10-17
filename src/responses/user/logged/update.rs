/// represents one of possible responses returned from updating logged user's data (PATCH /users/me)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum LoggedUserUpdateResponse {
    /// ok
    Ok,
    /// token sent is invalid - in wrong format or pointing to a user that doesn't exist
    InvalidToken,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
