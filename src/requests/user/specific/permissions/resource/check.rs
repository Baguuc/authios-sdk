#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecificUserCheckResourcePermissionRequest {
    pub service_id: String,
    pub resource_type: String,
    pub resource_id: u32,
    pub permission_name: String
}
