#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserUpdateRequest {
    pub login: Option<String>,
    pub password: Option<String>
}
