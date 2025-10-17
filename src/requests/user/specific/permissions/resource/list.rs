#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecificUserListResourcePermissionsRequest {
    pub service_id: String,
    pub resource_type: String,
    // pagination
    pub page_number: Option<u32>,
    // partial fetching values
    pub get_page_number: Option<bool>,
    pub get_total_page_count: Option<bool>,
    pub get_service_id: Option<bool>,
    pub get_resource_type: Option<bool>,
    pub get_resource_id: Option<bool>,
    pub get_permission_names: Option<bool>
}
