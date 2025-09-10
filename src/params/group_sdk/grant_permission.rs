#[derive(serde::Serialize)]
pub struct GrantPermissionParams {
    pub group_name: String,
    pub permission_name: String,
    pub token: String
}
