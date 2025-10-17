/// represents one of possible responses returned from login route (POST /users/me)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum AllUserLoginResponse {
    /// ok, token returned
    Ok {
        token: String
    },
    /// user not found
    UserNotFound,
    /// wrong password provided
    WrongPassword,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
