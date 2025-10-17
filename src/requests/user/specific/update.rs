#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecificUserUpdateRequest {
    pub login: Option<String>,
    pub password: Option<String>
}
