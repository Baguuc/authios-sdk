#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserGetInfoRequest {
    // partial fetching values
    pub get_id: bool,
    pub get_login: bool,
    pub get_password_hash: bool
}
