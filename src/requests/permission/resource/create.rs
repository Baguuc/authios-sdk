#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourcePermissionCreateRequest {
    pub service_id: String,
    pub resource_type: String,
    pub permission_name: String
}
