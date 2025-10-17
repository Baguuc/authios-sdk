#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecificUserCheckServicePermissionRequest {
    /// id of the service to check the permission for
    pub service_id: String
}
