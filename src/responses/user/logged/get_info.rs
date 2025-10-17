/// represents one of possible responses returned from getting logged user's info (GET /users/me)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum LoggedUserGetInfoResponse {
    /// ok
    Ok {
        user: UserData
    },
    /// token sent is invalid - in wrong format or pointing to a user that doesn't exist
    InvalidToken,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData {
    pub id: Option<i32>,
    pub login: Option<String>,
    pub password_hash: Option<String>
}
