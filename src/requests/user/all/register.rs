#[derive(serde::Serialize, serde::Deserialize)]
pub struct AllUserRegisterRequest {
    pub login: String,
    pub password: String
}
