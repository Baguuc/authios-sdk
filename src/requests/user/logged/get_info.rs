#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserGetInfoRequest {
    // partial fetching values
    pub get_id: Option<bool>,
    pub get_login: Option<bool>,
    pub get_password_hash: Option<bool>
}
