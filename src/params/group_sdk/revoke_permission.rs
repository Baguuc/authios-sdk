#[derive(serde::Serialize)]
pub struct RevokePermissionParams {
    pub group_name: String,
    pub permission_name: String,
    pub token: String
}
