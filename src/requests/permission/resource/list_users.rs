#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourcePermissionListUsersRequest {
    pub service_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub page_number: u32,
    pub get_page_number: bool,
    pub get_id: bool,
    pub get_login: bool,
    pub get_password_hash: bool
}
