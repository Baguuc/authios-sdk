#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserCheckResourcePermissionRequest {
    pub service_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub permission_name: String
}
