#[derive(serde::Serialize, serde::Deserialize)]
pub struct AllUserLoginRequest {
    pub login: String,
    pub password: String
}
