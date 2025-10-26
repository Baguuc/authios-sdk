#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserListResourcePermissionsRequest {
    pub service_id: String,
    pub resource_type: String,
    // pagination
    pub page_number: u32,
    // partial fetching values
    pub get_page_number: bool,
    pub get_service_id: bool,
    pub get_resource_type: bool,
    pub get_resource_id: bool,
    pub get_permission_names: bool
}
