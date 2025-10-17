#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoggedUserCheckServicePermissionRequest {
    /// id of the service to check the permission for
    pub service_id: String
}
